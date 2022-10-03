use std::{collections::{HashMap, HashSet}, cmp::Ordering};

use bson::{Document, doc};
use chrono::Utc;
use itertools::Itertools;
use mongodb::{Collection, options::FindOptions};
use futures::stream::{StreamExt, TryStreamExt};
use serde_derive::{Serialize, Deserialize};


use crate::{parser, common::SERVICE_NAME, context::AppContext, models::{self, SubmitMetadata, RankingEntry, VotingTrendItem, RankingQueryResponse, RankingGlobal, CachedRankingEntry, CachedRankingGlobal, CPItem, CPRankingQueryResponse, CPRankingEntry, CachedCPRankingEntry, GlobalStats, CompletionRate, CompletionRateItem, SinglePaperItem, CachedQuestionItem, CachedQuestionAnswerItem, CachedQuestionEntry, QueryQuestionnaireResponse, CovoteItem, CachedCovote}, service_error::ServiceError};

use phf::phf_map;

static KIND_MAPPING: phf::Map<&'static str, &'static str> = phf_map! {
    "old" => "旧作",
    "new" => "新作",
	"CD" => "专辑",
	"book" => "出版物",
	"others" => "其他",
	"other" => "其他",
	"game" => "游戏"
};

#[derive(Clone, Serialize, Deserialize)]
struct PartialVoteCharEntry {
	pub name: String,
	pub reason: Option<String>
}

#[derive(Clone, Serialize, Deserialize)]
struct PartialVotePaperEntry {
	pub opt: Vec<String>
}

#[derive(Clone, Serialize, Deserialize)]
struct PartialVote {
	pub q11011: PartialVotePaperEntry,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub chars: Option<Vec<PartialVoteCharEntry>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub chars_first: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub musics: Option<Vec<PartialVoteCharEntry>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub musics_first: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub cps: Option<Vec<CPItem>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub cps_first: Option<Vec<CPItem>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub chars_meta: Option<SubmitMetadata>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub musics_meta: Option<SubmitMetadata>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub cps_meta: Option<SubmitMetadata>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub doujins: Option<Vec<Document>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub paper_meta: Option<SubmitMetadata>,
}

pub fn process_query(query: Option<String>) -> Result<(Option<Document>, String), ServiceError> {
	if let Some(q) = query {
		let parsed = parser::generate_mongodb_query(&q).map_err(|e| ServiceError::from_dyn_error(SERVICE_NAME, e))?;
		let cache_key = format!("{:?}", parsed);
		Ok((Some(parsed), cache_key))
	} else {
		Ok((None, "none".to_string()))
	}
}


pub async fn chars_reasons(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32, rank: i32) ->  Result<models::ReasonsResponse, ServiceError> {
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year
	};
	let cached_global = ctx.chars_global_cache_coll.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_global) = cached_global {
		let opt = FindOptions::builder().skip(Some((std::cmp::max(rank, 1) - 1) as u64)).limit(1).build();
		let mut cached_entries = ctx.chars_entry_cache_coll.find(cache_query, Some(opt)).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let mut entries = Vec::with_capacity(300);
		while let Some(Ok(entry)) = cached_entries.next().await {
			entries.push(entry.entry);
		}
		// build response
		if entries.len() != 0 {
			let resp = models::ReasonsResponse {
				reasons: entries[0].reasons.clone()
			};
			Ok(resp)
		} else {
			let resp = models::ReasonsResponse {
				reasons: vec![]
			};
			Ok(resp)
		}
	} else {
		let resp = models::ReasonsResponse {
			reasons: vec![]
		};
		Ok(resp)
	}
}

pub async fn chars_trend(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32, name: String) ->  Result<models::TrendResponse, ServiceError> {
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year,
		"entry.name": name
	};
	let cached_entry = ctx.chars_entry_cache_coll.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_entry) = cached_entry {
		let resp = models::TrendResponse {
			trend: cached_entry.entry.trend,
			trend_first: cached_entry.entry.trend_first
		};
		Ok(resp)
	} else {
		let resp = models::TrendResponse {
			trend: vec![],
			trend_first: vec![]
		};
		Ok(resp)
	}
}

pub async fn chars_ranking(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32) ->  Result<models::RankingQueryResponse, ServiceError> {
	let empty_query = query.is_none();
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// lock query
	let lockid = format!("lock-chars_ranking-{}", cache_key);
	let guard = ctx.lock.acquire_async(lockid.as_bytes(), 60 * 1000).await;
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year
	};
	let cached_global = ctx.chars_global_cache_coll.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_global) = cached_global {
		let mut cached_entries = ctx.chars_entry_cache_coll.find(cache_query, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let mut entries = Vec::with_capacity(300);
		while let Some(Ok(entry)) = cached_entries.next().await {
			entries.push(entry.entry);
		}
		// build response
		let resp = RankingQueryResponse {
			entries: entries,
			global: cached_global.global
		};
		return Ok(resp);
	};
	// else
	let mut votes_cursor = ctx.votes_coll.find(filter, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	let mut hrs_bins: HashMap<String, Vec<i32>> = HashMap::with_capacity(300);
	let mut hrs_bins_first: HashMap<String, Vec<i32>> = HashMap::with_capacity(300);
	let mut reasons: HashMap<String, Vec<String>> = HashMap::with_capacity(300);
	let mut per_char_vote_count: HashMap<String, i32> = HashMap::with_capacity(300);
	let mut per_char_vote_first_count: HashMap<String, i32> = HashMap::with_capacity(300);
	let mut per_char_male_vote_count: HashMap<String, i32> = HashMap::with_capacity(300);
	let mut per_char_female_vote_count: HashMap<String, i32> = HashMap::with_capacity(300);
	let mut total_votes = 0i32;
	let mut total_first_votes = 0i32;
	let mut total_male = 0i32;
	let mut total_female = 0i32;
	while let Some(Ok(vote)) = votes_cursor.next().await {
		let pv: PartialVote = bson::from_document(vote).unwrap();
		if pv.chars.is_none() || pv.chars_meta.is_none() {
			continue;
		}
		let chs = pv.chars.as_ref().unwrap();
		if chs.len() == 0 {
			continue;
		}
		let hrs_diff = (pv.chars_meta.as_ref().unwrap().created_at.to_chrono() - vote_start.to_chrono()).num_hours() as usize;
		if let Some(f) = &pv.chars_first {
			if f.len() != 0 {
				total_first_votes += 1;
				*per_char_vote_first_count.entry(f[0].clone()).or_default() += 1;
				if !hrs_bins_first.contains_key(&f[0]) {
					hrs_bins_first.insert(f[0].clone(), vec![0i32; 24 * 30]);
				}
				let trend_hrs_bins = hrs_bins_first.get_mut(&f[0]).unwrap();
				trend_hrs_bins[hrs_diff] += 1;
			}
		}
		total_votes += 1;
		let is_male = if pv.q11011.opt[0] == "1101101" {
			total_male += 1;
			true
		} else {
			total_female += 1;
			false
		};
		for ch in chs {
			*per_char_vote_count.entry(ch.name.clone()).or_default() += 1;
			if !hrs_bins.contains_key(&ch.name) {
				hrs_bins.insert(ch.name.clone(), vec![0i32; 24 * 30]);
			}
			if !reasons.contains_key(&ch.name) {
				reasons.insert(ch.name.clone(), Vec::with_capacity(100));
			}
			let trend_hrs_bins = hrs_bins.get_mut(&ch.name).unwrap();
			trend_hrs_bins[hrs_diff] += 1;
			if is_male {
				*per_char_male_vote_count.entry(ch.name.clone()).or_default() += 1;
			} else {
				*per_char_female_vote_count.entry(ch.name.clone()).or_default() += 1;
			}
			if let Some(r) = &ch.reason {
				reasons.get_mut(&ch.name).unwrap().push(r.clone());
			}
		}
	}
	let mut chars_result = Vec::with_capacity(300);
	let mut per_char_vote_count_vec: Vec<(&String, &i32)> = per_char_vote_count.iter().collect();
	let mut per_char_vote_count_count_only_vec: Vec<i32> = per_char_vote_count.iter().map(|(a, b)| *b).collect();
	per_char_vote_count_count_only_vec.sort();
	per_char_vote_count_vec.sort_by(
		|a, b| {
			if b.1 == a.1 {
				per_char_vote_first_count.get(b.0).cloned().unwrap_or_default().cmp(&per_char_vote_first_count.get(a.0).cloned().unwrap_or_default())
			} else {
				b.1.cmp(a.1)
			}
		}
	);
	let mut rank = 1;
	if total_male == 0 {
		total_male = 1;
	}
	if total_female == 0 {
		total_female = 1;
	}
	if total_first_votes == 0 {
		total_first_votes = 1;
	}
	let mut display_rank = 1;
	let mut last_votes = 0;
	let all_chars = {
		let mut cursor = ctx.all_chars.find(doc!{"vote_year": vote_year}, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let mut items = Vec::with_capacity(600);
		while let Some(Ok(vote)) = cursor.next().await {
			items.push(vote);
		}
		items
	};
	let all_chars_map: HashMap<_, _> = all_chars
		.iter()
		.map(|x| {
			(x.name.clone(), x)
		})
		.collect();
	for (ch, _) in per_char_vote_count_vec {
		let trend = hrs_bins
			.get(ch)
			.unwrap()
			.iter()
			.enumerate()
			.filter(|(_, cnt)| {**cnt != 0})
			.map(|(hrs, cnt)| {
				VotingTrendItem { hrs: hrs as _, cnt: *cnt }
			})
			.collect::<Vec<_>>();
		let trend_first = if hrs_bins_first.contains_key(ch) {
			hrs_bins_first
			.get(ch)
			.unwrap()
			.iter()
			.enumerate()
			.filter(|(_, cnt)| {**cnt != 0})
			.map(|(hrs, cnt)| {
				VotingTrendItem { hrs: hrs as _, cnt: *cnt }
			})
			.collect::<Vec<_>>()
		} else {
			vec![]
		};
		let mut entry = RankingEntry {
			rank,
			display_rank,
			name: ch.clone(),
			vote_count: *per_char_vote_count.get(ch).unwrap_or(&0),
			first_vote_count: *per_char_vote_first_count.get(ch).unwrap_or(&0),
			first_vote_percentage: *per_char_vote_first_count.get(ch).unwrap_or(&0) as f64 / *per_char_vote_count.get(ch).unwrap_or(&0) as f64,
			first_vote_count_weighted: per_char_vote_count.get(ch).unwrap_or(&0) + per_char_vote_first_count.get(ch).unwrap_or(&0),
			character_type: if all_chars_map.contains_key(ch) { all_chars_map.get(ch).unwrap().kind.iter().map(|f| KIND_MAPPING.get(f).unwrap()).join(",").into() } else { "未知".into() },
			character_origin: if all_chars_map.contains_key(ch) { all_chars_map.get(ch).unwrap().work.join(",").into() } else { "未知".into() },
			first_appearance: if all_chars_map.contains_key(ch) { all_chars_map.get(ch).unwrap().date.to_string() } else { "未知".into() },
			name_jpn: if all_chars_map.contains_key(ch) { all_chars_map.get(ch).unwrap().origname.clone() } else { "未知".into() },
			vote_percentage: *per_char_vote_count.get(ch).unwrap_or(&0) as f64 / total_votes as f64,
			first_percentage: *per_char_vote_first_count.get(ch).unwrap_or(&0) as f64 / total_first_votes as f64,
			male_vote_count: *per_char_male_vote_count.get(ch).unwrap_or(&0),
			male_percentage_per_char: *per_char_male_vote_count.get(ch).unwrap_or(&0) as f64 / *per_char_vote_count.get(ch).unwrap_or(&0) as f64,
			male_percentage_per_total: *per_char_male_vote_count.get(ch).unwrap_or(&0) as f64 / total_male as f64,
			female_vote_count: *per_char_female_vote_count.get(ch).unwrap_or(&0),
			female_percentage_per_char: *per_char_female_vote_count.get(ch).unwrap_or(&0) as f64 / *per_char_vote_count.get(ch).unwrap_or(&0) as f64,
			female_percentage_per_total: *per_char_female_vote_count.get(ch).unwrap_or(&0) as f64 / total_female as f64,
			rank_last_1: 0,
			rank_last_2: 0,
			vote_count_last_1: 0,
			vote_count_last_2: 0,
			first_vote_count_last_1: 0,
			first_vote_count_last_2: 0,
			first_vote_percentage_last_1: 0.0,
			first_vote_percentage_last_2: 0.0,
			vote_percentage_last_1: 0.0,
			vote_percentage_last_2: 0.0,
			trend,
			trend_first,
			reasons: reasons.get(ch).unwrap_or(&vec![]).clone()
		};
		let cur_votes = entry.vote_count;
		if last_votes != cur_votes {
			display_rank = rank;
			entry.display_rank = display_rank;
		}
		rank += 1;
		last_votes = cur_votes;
		chars_result.push(entry);
	};
	if empty_query {
		let all_chars: HashSet<String> = HashSet::from_iter(all_chars.iter().map(|x| x.name.clone()));
		let voted_chars: HashSet<String> = HashSet::from_iter(chars_result.iter().map(|x| x.name.clone()));
		display_rank = rank;
		for x in all_chars.difference(&voted_chars) {
			let entry = RankingEntry {
				rank,
				display_rank,
				name: x.clone(),
				vote_count: 0,
				first_vote_count: 0,
				first_vote_percentage: 0f64,
				first_vote_count_weighted: 0,
				character_type: "todo".to_owned(),
				character_origin: "todo".to_owned(),
				first_appearance: "todo".to_owned(),
				name_jpn: "todo".to_owned(),
				vote_percentage: 0f64,
				first_percentage: 0f64,
				male_vote_count: 0,
				male_percentage_per_char: 0f64,
				male_percentage_per_total: 0f64,
				female_vote_count: 0,
				female_percentage_per_char: 0f64,
				female_percentage_per_total: 0f64,
				trend: vec![],
				trend_first: vec![],
				reasons: vec![],
				rank_last_1: 0,
				rank_last_2: 0,
				vote_count_last_1: 0,
				vote_count_last_2: 0,
				first_vote_count_last_1: 0,
				first_vote_count_last_2: 0,
				first_vote_percentage_last_1: 0.0,
				first_vote_percentage_last_2: 0.0,
				vote_percentage_last_1: 0.0,
				vote_percentage_last_2: 0.0,
			};
			rank += 1;
			chars_result.push(entry);
		}
	};
	let num_char = per_char_vote_count_count_only_vec.len();
	let avg = if num_char == 0 { 0f64 } else { total_votes as f64 / num_char as f64 };
	let median = if num_char % 2 == 0 {
		if num_char != 0 {
			0.5f64 * (
				per_char_vote_count_count_only_vec[num_char / 2 - 1] as f64 +
				per_char_vote_count_count_only_vec[num_char / 2] as f64
			)
		} else {
			0f64
		}
	} else {
		per_char_vote_count_count_only_vec[num_char / 2] as f64
	};
	let global = RankingGlobal {
		total_unique_items: num_char as _,
		total_first: total_first_votes,
		total_votes,
		average_votes_per_item: avg,
		median_votes_per_item: median,
	};

	// build cache
	let cached_entries = chars_result
		.iter()
		.map(|f| {
			CachedRankingEntry {
				key: cache_key.clone(),
				vote_year,
				entry: f.clone()
			}
		})
		.collect::<Vec<_>>();
	let cached_global = CachedRankingGlobal {
		key: cache_key.clone(),
		vote_year,
		global: global.clone()
	};
	if cached_entries.len() != 0 {
		ctx.chars_entry_cache_coll.insert_many(cached_entries, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		ctx.chars_global_cache_coll.insert_one(cached_global, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	}
	// build response
	let resp = RankingQueryResponse {
		entries: chars_result,
		global
	};
	Ok(resp)
}

pub async fn musics_reasons(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32, rank: i32) ->  Result<models::ReasonsResponse, ServiceError> {
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year
	};
	let cached_global = ctx.musics_global_cache_coll.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_global) = cached_global {
		let opt = FindOptions::builder().skip(Some((std::cmp::max(rank, 1) - 1) as u64)).limit(1).build();
		let mut cached_entries = ctx.musics_entry_cache_coll.find(cache_query, Some(opt)).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let mut entries = Vec::with_capacity(300);
		while let Some(Ok(entry)) = cached_entries.next().await {
			entries.push(entry.entry);
		}
		// build response
		if entries.len() != 0 {
			let resp = models::ReasonsResponse {
				reasons: entries[0].reasons.clone()
			};
			Ok(resp)
		} else {
			let resp = models::ReasonsResponse {
				reasons: vec![]
			};
			Ok(resp)
		}
	} else {
		let resp = models::ReasonsResponse {
			reasons: vec![]
		};
		Ok(resp)
	}
}

pub async fn musics_trend(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32, name: String) ->  Result<models::TrendResponse, ServiceError> {
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year,
		"entry.name": name
	};
	let cached_entry = ctx.musics_entry_cache_coll.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_entry) = cached_entry {
		let resp = models::TrendResponse {
			trend: cached_entry.entry.trend,
			trend_first: cached_entry.entry.trend_first,
		};
		Ok(resp)
	} else {
		let resp = models::TrendResponse {
			trend: vec![],
			trend_first: vec![]
		};
		Ok(resp)
	}
}

pub async fn musics_ranking(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32) ->  Result<models::RankingQueryResponse, ServiceError> {
	let empty_query = query.is_none();
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// lock query
	let lockid = format!("lock-musics_ranking-{}", cache_key);
	let guard = ctx.lock.acquire_async(lockid.as_bytes(), 60 * 1000).await;
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year
	};
	let cached_global = ctx.musics_global_cache_coll.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_global) = cached_global {
		let mut cached_entries = ctx.musics_entry_cache_coll.find(cache_query, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let mut entries = Vec::with_capacity(300);
		while let Some(Ok(entry)) = cached_entries.next().await {
			entries.push(entry.entry);
		}
		// build response
		let resp = RankingQueryResponse {
			entries: entries,
			global: cached_global.global
		};
		return Ok(resp);
	};
	// else
	let mut votes_cursor = ctx.votes_coll.find(filter, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	let mut hrs_bins: HashMap<String, Vec<i32>> = HashMap::with_capacity(300);
	let mut hrs_bins_first: HashMap<String, Vec<i32>> = HashMap::with_capacity(300);
	let mut reasons: HashMap<String, Vec<String>> = HashMap::with_capacity(300);
	let mut per_music_vote_count: HashMap<String, i32> = HashMap::with_capacity(300);
	let mut per_music_vote_first_count: HashMap<String, i32> = HashMap::with_capacity(300);
	let mut per_music_male_vote_count: HashMap<String, i32> = HashMap::with_capacity(300);
	let mut per_music_female_vote_count: HashMap<String, i32> = HashMap::with_capacity(300);
	let mut total_votes = 0i32;
	let mut total_first_votes = 0i32;
	let mut total_male = 0i32;
	let mut total_female = 0i32;
	while let Some(Ok(vote)) = votes_cursor.next().await {
		let pv: PartialVote = bson::from_document(vote).unwrap();
		if pv.musics.is_none() || pv.musics_meta.is_none() {
			continue;
		}
		let chs = pv.musics.as_ref().unwrap();
		if chs.len() == 0 {
			continue;
		}
		let hrs_diff = (pv.musics_meta.as_ref().unwrap().created_at.to_chrono() - vote_start.to_chrono()).num_hours() as usize;
		if let Some(f) = &pv.musics_first {
			if f.len() != 0 {
				total_first_votes += 1;
				*per_music_vote_first_count.entry(f[0].clone()).or_default() += 1;
				if !hrs_bins_first.contains_key(&f[0]) {
					hrs_bins_first.insert(f[0].clone(), vec![0i32; 24 * 30]);
				}
				let trend_hrs_bins = hrs_bins_first.get_mut(&f[0]).unwrap();
				trend_hrs_bins[hrs_diff] += 1;
			}
		}
		total_votes += 1;
		let is_male = if pv.q11011.opt[0] == "1101101" {
			total_male += 1;
			true
		} else {
			total_female += 1;
			false
		};
		for ch in chs {
			*per_music_vote_count.entry(ch.name.clone()).or_default() += 1;
			if !hrs_bins.contains_key(&ch.name) {
				hrs_bins.insert(ch.name.clone(), vec![0i32; 24 * 30]);
			}
			if !reasons.contains_key(&ch.name) {
				reasons.insert(ch.name.clone(), Vec::with_capacity(100));
			}
			let trend_hrs_bins = hrs_bins.get_mut(&ch.name).unwrap();
			trend_hrs_bins[hrs_diff] += 1;
			if is_male {
				*per_music_male_vote_count.entry(ch.name.clone()).or_default() += 1;
			} else {
				*per_music_female_vote_count.entry(ch.name.clone()).or_default() += 1;
			}
			if let Some(r) = &ch.reason {
				reasons.get_mut(&ch.name).unwrap().push(r.clone());
			}
		}
	}
	let mut musics_result = Vec::with_capacity(300);
	let mut per_music_vote_count_vec: Vec<(&String, &i32)> = per_music_vote_count.iter().collect();
	let mut per_music_vote_count_count_only_vec: Vec<i32> = per_music_vote_count.iter().map(|(a, b)| *b).collect();
	per_music_vote_count_count_only_vec.sort();
	per_music_vote_count_vec.sort_by(
		|a, b| {
			if b.1 == a.1 {
				per_music_vote_first_count.get(b.0).cloned().unwrap_or_default().cmp(&per_music_vote_first_count.get(a.0).cloned().unwrap_or_default())
			} else {
				b.1.cmp(a.1)
			}
		}
	);
	let mut rank = 1;
	if total_male == 0 {
		total_male = 1;
	}
	if total_female == 0 {
		total_female = 1;
	}
	if total_first_votes == 0 {
		total_first_votes = 1;
	}
	let mut display_rank = 1;
	let mut last_votes = 0;
	let all_musics = {
		let mut cursor = ctx.all_musics.find(doc!{"vote_year": vote_year}, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let mut items = Vec::with_capacity(600);
		while let Some(Ok(vote)) = cursor.next().await {
			items.push(vote);
		}
		items
	};
	let all_musics_map: HashMap<_, _> = all_musics
		.iter()
		.map(|x| {
			(x.name.clone(), x)
		})
		.collect();
	for (ch, _) in per_music_vote_count_vec {
		let trend = hrs_bins
			.get(ch)
			.unwrap()
			.iter()
			.enumerate()
			.filter(|(_, cnt)| {**cnt != 0})
			.map(|(hrs, cnt)| {
				VotingTrendItem { hrs: hrs as _, cnt: *cnt }
			})
			.collect::<Vec<_>>();
		let trend_first = if hrs_bins_first.contains_key(ch) {
			hrs_bins_first
			.get(ch)
			.unwrap()
			.iter()
			.enumerate()
			.filter(|(_, cnt)| {**cnt != 0})
			.map(|(hrs, cnt)| {
				VotingTrendItem { hrs: hrs as _, cnt: *cnt }
			})
			.collect::<Vec<_>>()
		} else {
			vec![]
		};
		let mut entry = RankingEntry {
			rank,
			display_rank,
			name: ch.clone(),
			vote_count: *per_music_vote_count.get(ch).unwrap_or(&0),
			first_vote_count: *per_music_vote_first_count.get(ch).unwrap_or(&0),
			first_vote_percentage: *per_music_vote_first_count.get(ch).unwrap_or(&0) as f64 / *per_music_vote_count.get(ch).unwrap_or(&0) as f64,
			first_vote_count_weighted: per_music_vote_count.get(ch).unwrap_or(&0) + per_music_vote_first_count.get(ch).unwrap_or(&0),
			character_type: if all_musics_map.contains_key(ch) { all_musics_map.get(ch).unwrap().kind.iter().map(|f| KIND_MAPPING.get(f).unwrap()).join(",").into() } else { "未知".into() },
			character_origin: if all_musics_map.contains_key(ch) { all_musics_map.get(ch).unwrap().work.join(",").into() } else { "未知".into() },
			first_appearance: if all_musics_map.contains_key(ch) { all_musics_map.get(ch).unwrap().date.to_string() } else { "未知".into() },
			name_jpn: if all_musics_map.contains_key(ch) { all_musics_map.get(ch).unwrap().origname.clone() } else { "未知".into() },
			vote_percentage: *per_music_vote_count.get(ch).unwrap_or(&0) as f64 / total_votes as f64,
			first_percentage: *per_music_vote_first_count.get(ch).unwrap_or(&0) as f64 / total_first_votes as f64,
			male_vote_count: *per_music_male_vote_count.get(ch).unwrap_or(&0),
			male_percentage_per_char: *per_music_male_vote_count.get(ch).unwrap_or(&0) as f64 / *per_music_vote_count.get(ch).unwrap_or(&0) as f64,
			male_percentage_per_total: *per_music_male_vote_count.get(ch).unwrap_or(&0) as f64 / total_male as f64,
			female_vote_count: *per_music_female_vote_count.get(ch).unwrap_or(&0),
			female_percentage_per_char: *per_music_female_vote_count.get(ch).unwrap_or(&0) as f64 / *per_music_vote_count.get(ch).unwrap_or(&0) as f64,
			female_percentage_per_total: *per_music_female_vote_count.get(ch).unwrap_or(&0) as f64 / total_female as f64,
			trend,
			trend_first,
			reasons: reasons.get(ch).unwrap_or(&vec![]).clone(),
			rank_last_1: 0,
			rank_last_2: 0,
			vote_count_last_1: 0,
			vote_count_last_2: 0,
			first_vote_count_last_1: 0,
			first_vote_count_last_2: 0,
			first_vote_percentage_last_1: 0.0,
			first_vote_percentage_last_2: 0.0,
			vote_percentage_last_1: 0.0,
			vote_percentage_last_2: 0.0,
		};
		let cur_votes = (entry.vote_count);
		if last_votes != cur_votes {
			display_rank = rank;
			entry.display_rank = display_rank;
		}
		rank += 1;
		last_votes = cur_votes;
		musics_result.push(entry);
	};
	if empty_query {
		let all_musics: HashSet<String> = HashSet::from_iter(all_musics.iter().map(|x| x.name.clone()));
		let voted_musics: HashSet<String> = HashSet::from_iter(musics_result.iter().map(|x| x.name.clone()));
		display_rank = rank;
		for x in all_musics.difference(&voted_musics) {
			let entry = RankingEntry {
				rank,
				display_rank,
				name: x.clone(),
				vote_count: 0,
				first_vote_count: 0,
				first_vote_percentage: 0f64,
				first_vote_count_weighted: 0,
				character_type: "todo".to_owned(),
				character_origin: "todo".to_owned(),
				first_appearance: "todo".to_owned(),
				name_jpn: "todo".to_owned(),
				vote_percentage: 0f64,
				first_percentage: 0f64,
				male_vote_count: 0,
				male_percentage_per_char: 0f64,
				male_percentage_per_total: 0f64,
				female_vote_count: 0,
				female_percentage_per_char: 0f64,
				female_percentage_per_total: 0f64,
				trend: vec![],
				trend_first: vec![],
				reasons: vec![],
				rank_last_1: 0,
				rank_last_2: 0,
				vote_count_last_1: 0,
				vote_count_last_2: 0,
				first_vote_count_last_1: 0,
				first_vote_count_last_2: 0,
				first_vote_percentage_last_1: 0.0,
				first_vote_percentage_last_2: 0.0,
				vote_percentage_last_1: 0.0,
				vote_percentage_last_2: 0.0,
			};
			rank += 1;
			musics_result.push(entry);
		}
	};
	let num_music = per_music_vote_count_count_only_vec.len();
	let avg = if num_music == 0 { 0f64 } else { total_votes as f64 / num_music as f64 };
	let median = if num_music % 2 == 0 {
		if num_music != 0 {
			0.5f64 * (
				per_music_vote_count_count_only_vec[num_music / 2 - 1] as f64 +
				per_music_vote_count_count_only_vec[num_music / 2] as f64
			)
		} else {
			0f64
		}
	} else {
		per_music_vote_count_count_only_vec[num_music / 2] as f64
	};
	let global = RankingGlobal {
		total_unique_items: num_music as _,
		total_first: total_first_votes,
		total_votes,
		average_votes_per_item: avg,
		median_votes_per_item: median,
	};

	// build cache
	let cached_entries = musics_result
		.iter()
		.map(|f| {
			CachedRankingEntry {
				key: cache_key.clone(),
				vote_year,
				entry: f.clone()
			}
		})
		.collect::<Vec<_>>();
	let cached_global = CachedRankingGlobal {
		key: cache_key.clone(),
		vote_year,
		global: global.clone()
	};
	if cached_entries.len() != 0 {
		ctx.musics_entry_cache_coll.insert_many(cached_entries, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		ctx.musics_global_cache_coll.insert_one(cached_global, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	}
	// build response
	let resp = RankingQueryResponse {
		entries: musics_result,
		global
	};
	Ok(resp)
}

pub async fn cps_reasons(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32, rank: i32) ->  Result<models::ReasonsResponse, ServiceError> {
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year
	};
	let cached_global = ctx.cps_global_cache_coll.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_global) = cached_global {
		let opt = FindOptions::builder().skip(Some((std::cmp::max(rank, 1) - 1) as u64)).limit(1).build();
		let mut cached_entries = ctx.cps_entry_cache_coll.find(cache_query, Some(opt)).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let mut entries = Vec::with_capacity(300);
		while let Some(Ok(entry)) = cached_entries.next().await {
			entries.push(entry.entry);
		}
		// build response
		if entries.len() != 0 {
			let resp = models::ReasonsResponse {
				reasons: entries[0].reasons.clone()
			};
			Ok(resp)
		} else {
			let resp = models::ReasonsResponse {
				reasons: vec![]
			};
			Ok(resp)
		}
	} else {
		let resp = models::ReasonsResponse {
			reasons: vec![]
		};
		Ok(resp)
	}
}

pub async fn cps_ranking(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32) ->  Result<models::CPRankingQueryResponse, ServiceError> {
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// lock query
	let lockid = format!("lock-cps_ranking-{}", cache_key);
	let guard = ctx.lock.acquire_async(lockid.as_bytes(), 60 * 1000).await;
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year
	};
	let cached_global = ctx.cps_global_cache_coll.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_global) = cached_global {
		let mut cached_entries = ctx.cps_entry_cache_coll.find(cache_query, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		let mut entries = Vec::with_capacity(1000);
		while let Some(Ok(entry)) = cached_entries.next().await {
			entries.push(entry.entry);
		}
		// build response
		let resp = CPRankingQueryResponse {
			entries: entries,
			global: cached_global.global
		};
		return Ok(resp);
	};
	// else
	let mut votes_cursor = ctx.votes_coll.find(filter, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	let mut hrs_bins: HashMap<CPItem, Vec<i32>> = HashMap::with_capacity(300);
	let mut reasons: HashMap<CPItem, Vec<String>> = HashMap::with_capacity(1000);
	let mut a_active: HashMap<CPItem, i32> = HashMap::with_capacity(1000);
	let mut b_active: HashMap<CPItem, i32> = HashMap::with_capacity(1000);
	let mut c_active: HashMap<CPItem, i32> = HashMap::with_capacity(1000);
	let mut none_active: HashMap<CPItem, i32> = HashMap::with_capacity(1000);
	let mut per_cp_vote_count: HashMap<CPItem, i32> = HashMap::with_capacity(1000);
	let mut per_cp_vote_first_count: HashMap<CPItem, i32> = HashMap::with_capacity(1000);
	let mut per_cp_male_vote_count: HashMap<CPItem, i32> = HashMap::with_capacity(1000);
	let mut per_cp_female_vote_count: HashMap<CPItem, i32> = HashMap::with_capacity(1000);
	let mut total_votes = 0i32;
	let mut total_first_votes = 0i32;
	let mut total_male = 0i32;
	let mut total_female = 0i32;
	while let Some(Ok(vote)) = votes_cursor.next().await {
		let pv: PartialVote = bson::from_document(vote).unwrap();
		if pv.cps.is_none() || pv.cps_meta.is_none() {
			continue;
		}
		if let Some(f) = &pv.cps_first {
			if f.len() != 0 {
				total_first_votes += 1;
				*per_cp_vote_first_count.entry(f[0].clone()).or_default() += 1;
			}
		}
		let chs = pv.cps.as_ref().unwrap();
		if chs.len() == 0 {
			continue;
		}
		total_votes += 1;
		let is_male = if pv.q11011.opt[0] == "1101101" {
			total_male += 1;
			true
		} else {
			total_female += 1;
			false
		};
		let hrs_diff = (pv.cps_meta.as_ref().unwrap().created_at.to_chrono() - vote_start.to_chrono()).num_hours() as usize;
		for ch in chs {
			*per_cp_vote_count.entry(ch.clone()).or_default() += 1;
			if !hrs_bins.contains_key(&ch) {
				hrs_bins.insert(ch.clone(), vec![0i32; 24 * 30]);
			}
			if !reasons.contains_key(&ch) {
				reasons.insert(ch.clone(), Vec::with_capacity(100));
			}
			let trend_hrs_bins = hrs_bins.get_mut(&ch).unwrap();
			trend_hrs_bins[hrs_diff] += 1;
			if is_male {
				*per_cp_male_vote_count.entry(ch.clone()).or_default() += 1;
			} else {
				*per_cp_female_vote_count.entry(ch.clone()).or_default() += 1;
			}
			if let Some(r) = &ch.reason {
				reasons.get_mut(&ch).unwrap().push(r.clone());
			}
			if let Some(active) = &ch.active {
				if *active == ch.a {
					*a_active.entry(ch.clone()).or_default() += 1;
				} else if *active == ch.b {
					*b_active.entry(ch.clone()).or_default() += 1;
				} else if let Some(c) = &ch.c {
					if *active == *c {
						*c_active.entry(ch.clone()).or_default() += 1;
					}
				}
			} else {
				*none_active.entry(ch.clone()).or_default() += 1;
			}
		}
	}
	let mut cps_result = Vec::with_capacity(300);
	let mut per_cp_vote_count_vec: Vec<(&CPItem, &i32)> = per_cp_vote_count.iter().filter(|(a, b)| **b > 1).collect();
	let mut per_cp_vote_count_count_only_vec: Vec<i32> = per_cp_vote_count.iter().filter(|(a, b)| **b > 1).map(|(a, b)| *b).collect();
	per_cp_vote_count_count_only_vec.sort();
	per_cp_vote_count_vec.sort_by(
		|a, b| {
			if b.1 == a.1 {
				per_cp_vote_first_count.get(b.0).cloned().unwrap_or_default().cmp(&per_cp_vote_first_count.get(a.0).cloned().unwrap_or_default())
			} else {
				b.1.cmp(a.1)
			}
		}
	);
	let mut rank = 1;
	if total_male == 0 {
		total_male = 1;
	}
	if total_female == 0 {
		total_female = 1;
	}
	if total_first_votes == 0 {
		total_first_votes = 1;
	}
	let mut display_rank = 1;
	let mut last_votes = 0;
	for (ch, _) in per_cp_vote_count_vec {
		let trend = hrs_bins
			.get(ch)
			.unwrap()
			.iter()
			.enumerate()
			.filter(|(_, cnt)| {**cnt != 0})
			.map(|(hrs, cnt)| {
				VotingTrendItem { hrs: hrs as _, cnt: *cnt }
			})
			.collect::<Vec<_>>();
		let mut entry = CPRankingEntry {
			rank,
			display_rank,
			cp: ch.clone(),
			a_active: *a_active.get(ch).unwrap_or(&0) as f64 / *per_cp_vote_count.get(ch).unwrap_or(&0) as f64,
			b_active: *b_active.get(ch).unwrap_or(&0) as f64 / *per_cp_vote_count.get(ch).unwrap_or(&0) as f64,
			c_active: *c_active.get(ch).unwrap_or(&0) as f64 / *per_cp_vote_count.get(ch).unwrap_or(&0) as f64,
			none_active: *none_active.get(ch).unwrap_or(&0) as f64 / *per_cp_vote_count.get(ch).unwrap_or(&0) as f64,
			vote_count: *per_cp_vote_count.get(ch).unwrap_or(&0),
			first_vote_count: *per_cp_vote_first_count.get(ch).unwrap_or(&0),
			first_vote_percentage: *per_cp_vote_first_count.get(ch).unwrap_or(&0) as f64 / *per_cp_vote_count.get(ch).unwrap_or(&0) as f64,
			first_vote_count_weighted: per_cp_vote_count.get(ch).unwrap_or(&0) + per_cp_vote_first_count.get(ch).unwrap_or(&0),
			vote_percentage: *per_cp_vote_count.get(ch).unwrap_or(&0) as f64 / total_votes as f64,
			first_percentage: *per_cp_vote_first_count.get(ch).unwrap_or(&0) as f64 / total_first_votes as f64,
			male_vote_count: *per_cp_male_vote_count.get(ch).unwrap_or(&0),
			male_percentage_per_char: *per_cp_male_vote_count.get(ch).unwrap_or(&0) as f64 / *per_cp_vote_count.get(ch).unwrap_or(&0) as f64,
			male_percentage_per_total: *per_cp_male_vote_count.get(ch).unwrap_or(&0) as f64 / total_male as f64,
			female_vote_count: *per_cp_female_vote_count.get(ch).unwrap_or(&0),
			female_percentage_per_char: *per_cp_female_vote_count.get(ch).unwrap_or(&0) as f64 / *per_cp_vote_count.get(ch).unwrap_or(&0) as f64,
			female_percentage_per_total: *per_cp_female_vote_count.get(ch).unwrap_or(&0) as f64 / total_female as f64,
			trend,
			reasons: reasons.get(ch).unwrap_or(&vec![]).clone()
		};
		let cur_votes = entry.vote_count;
		if last_votes != cur_votes {
			display_rank = rank;
			entry.display_rank = display_rank;
		}
		rank += 1;
		last_votes = cur_votes;
		cps_result.push(entry);
	};
	let num_cp = per_cp_vote_count_count_only_vec.len();
	let avg = if num_cp == 0 { 0f64 } else { total_votes as f64 / num_cp as f64 };
	let median = if num_cp % 2 == 0 {
		if num_cp != 0 {
			0.5f64 * (
				per_cp_vote_count_count_only_vec[num_cp / 2 - 1] as f64 +
				per_cp_vote_count_count_only_vec[num_cp / 2] as f64
			)
		} else {
			0f64
		}
	} else {
		per_cp_vote_count_count_only_vec[num_cp / 2] as f64
	};
	let global = RankingGlobal {
		total_unique_items: num_cp as _,
		total_first: total_first_votes,
		total_votes,
		average_votes_per_item: avg,
		median_votes_per_item: median,
	};

	// build cache
	let cached_entries = cps_result
		.iter()
		.map(|f| {
			CachedCPRankingEntry {
				key: cache_key.clone(),
				vote_year,
				entry: f.clone()
			}
		})
		.collect::<Vec<_>>();
	let cached_global = CachedRankingGlobal {
		key: cache_key.clone(),
		vote_year,
		global: global.clone()
	};
	if cached_entries.len() != 0 {
		ctx.cps_entry_cache_coll.insert_many(cached_entries, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		ctx.cps_global_cache_coll.insert_one(cached_global, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	}
	// build response
	let resp = CPRankingQueryResponse {
		entries: cps_result,
		global
	};
	Ok(resp)
}

pub async fn cps_trend(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32, name: String) ->  Result<models::TrendResponse, ServiceError> {
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year,
		"entry.name": name
	};
	let cached_entry = ctx.cps_entry_cache_coll.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_entry) = cached_entry {
		let resp = models::TrendResponse {
			trend: cached_entry.entry.trend,
			trend_first: vec![]
		};
		Ok(resp)
	} else {
		let resp = models::TrendResponse {
			trend: vec![],
			trend_first: vec![]
		};
		Ok(resp)
	}
}

pub async fn global_stats(ctx: &AppContext, vote_start: bson::DateTime, vote_year: i32) -> Result<models::GlobalStats, ServiceError> {
	let filter = doc! {"vote_year": vote_year};
	let cached_global = ctx.global_stats.find_one(filter.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_global) = cached_global {
		return Ok(cached_global);
	};
	// else
	let mut votes_cursor = ctx.votes_coll.find(filter, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	let mut gs = GlobalStats::default();
	gs.vote_year = vote_year;
	while let Some(Ok(vote)) = votes_cursor.next().await {
		let pv: PartialVote = bson::from_document(vote).unwrap();
		gs.num_vote += 1;
		if let Some(ch) = pv.chars {
			if ch.len() != 0 {
				gs.num_char += 1;
			}
		}
		if let Some(ch) = pv.musics {
			if ch.len() != 0 {
				gs.num_music += 1;
			}
		}
		if let Some(ch) = pv.cps {
			if ch.len() != 0 {
				gs.num_cp += 1;
			}
		}
		if let Some(ch) = pv.doujins {
			if ch.len() != 0 {
				gs.num_doujin += 1;
			}
		}
		let is_male = if pv.q11011.opt[0] == "1101101" {
			gs.num_male += 1;
			true
		} else {
			gs.num_female += 1;
			false
		};
	}
	ctx.global_stats.insert_one(gs.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	Ok(gs)
}

pub async fn completion_rates(ctx: &AppContext, vote_start: bson::DateTime, vote_year: i32) -> Result<models::CompletionRate, ServiceError> {
	let filter = doc! {"vote_year": vote_year};
	let cached_global = ctx.completion_rates.find_one(filter.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_global) = cached_global {
		return Ok(cached_global);
	};
	// else
	let mut votes_cursor = ctx.votes_coll.find(filter, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	let mut ret = CompletionRate::default();
	ret.vote_year = vote_year;
	let mut total_votes = 0i32;
	let mut question_count_map: HashMap<String, i32> = HashMap::new();
	while let Some(Ok(vote)) = votes_cursor.next().await {
		total_votes += 1;
		for (key, value) in vote.iter() {
			if key.chars().next().unwrap() == 'q' {
				if let Some(value_doc) = value.as_document() {
					if let Ok(opt) = value_doc.get_array(&"opt") {
						if opt.len() != 0 {
							*question_count_map.entry(key.clone()).or_insert(0) += 1;
						}
					}
				}
			}
		}
	}
	for (k, v) in question_count_map {
		let rate = (v as f64) / (total_votes as f64);
		ret.items.push(CompletionRateItem {
			name: k,
			rate,
			num_complete: v,
			total: total_votes
		});
	}
	ctx.completion_rates.insert_one(ret.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	Ok(ret)
}

pub async fn paper_result(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32, questions_of_interest: Vec<String>) ->  Result<models::QueryQuestionnaireResponse, ServiceError> {
	let questions_of_interest_set: HashSet<String> = HashSet::from_iter(questions_of_interest.clone().into_iter());
	// result of each question is cached in mongodb
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// step 1: find in cache
	let lockid = format!("lock-paper_result-{}", cache_key);
	let guard = ctx.lock.acquire_async(lockid.as_bytes(), 60 * 1000).await;
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year,
		"entry.question_id": {
			"$in": questions_of_interest
		}
	};
	let mut found_question_ids: HashSet<String> = HashSet::new();
	let mut result_entries = Vec::new();
	let mut cached = ctx.paper_result.find(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	while let Some(Ok(entry)) = cached.next().await {
		found_question_ids.insert(entry.entry.question_id.clone());
		result_entries.push(entry.entry);
	}
	// step 2: find out which questions are not cache
	let not_found_qids: HashSet<&String> = questions_of_interest_set.difference(&found_question_ids).collect::<_>();
	// step 3: build result for those questions
	if not_found_qids.len() != 0 {
		let mut question2answer_count: HashMap<String, HashMap<String, i32>> = HashMap::new();
		let mut question2answer_str: HashMap<String, Vec<String>> = HashMap::new();
		let mut question2cnt: HashMap<String, i32> = HashMap::new();
		let mut hrs_bins: HashMap<String, Vec<i32>> = HashMap::new();

		let mut votes_cursor = ctx.votes_coll.find(filter, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
		while let Some(Ok(vote)) = votes_cursor.next().await {
			for (key, value) in vote.iter() {
				if not_found_qids.contains(key) {
					if let Ok(vote_item) = bson::from_bson::<SinglePaperItem>(value.clone()) {
						let answer_count_map = question2answer_count.entry(key.clone()).or_insert(HashMap::new());
						*question2cnt.entry(key.clone()).or_insert(0) += 1;
						for opt_item in vote_item.opt.iter() {
							*answer_count_map.entry(opt_item.clone()).or_insert(0) += 1;
						}
						if vote_item.ans.len() != 0 {
							question2answer_str.entry(key.clone()).or_insert(Vec::new()).push(vote_item.ans.clone());
						}
						let pv: PartialVote = bson::from_document(vote.clone()).unwrap();
						if let Some(paper_meta) = pv.paper_meta {
							if !hrs_bins.contains_key(key) {
								hrs_bins.insert(key.clone(), vec![0i32; 24 * 30]);
							}
							let hrs_diff = (paper_meta.created_at.to_chrono() - vote_start.to_chrono()).num_hours() as usize;
							let trend_hrs_bins = hrs_bins.get_mut(key).unwrap();
							trend_hrs_bins[hrs_diff] += 1;
						}
					}
				}
			}
		}

		let mut result_items: Vec<CachedQuestionItem> = Vec::with_capacity(not_found_qids.len());
		for qid in not_found_qids {
			let mut item = CachedQuestionItem { question_id: qid.clone(), answers_cat: vec![], answers_str: vec![], total_answers: 0 };
			if let Some(answer_count) = question2answer_count.get(qid) {
				for (aid, cnt) in answer_count {
					item.answers_cat.push(CachedQuestionAnswerItem { aid: aid.clone(), votes: *cnt });
				}
			};
			if let Some(answer_str) = question2answer_str.get(qid) {
				item.answers_str = answer_str.clone();
			}
			if let Some(cnt) = question2cnt.get(qid) {
				item.total_answers = *cnt;
			}
			result_items.push(item);
		}
		result_entries.append(&mut result_items.clone());

		let cached_entries = result_items
			.iter()
			.map(|f| {
				CachedQuestionEntry {
					key: cache_key.clone(),
					vote_year: vote_year,
					entry: f.clone(),
					trend: {
						if let Some(trend) = hrs_bins.get(&f.question_id) {
							trend
								.iter()
								.enumerate()
								.filter(|(_, cnt)| {**cnt != 0})
								.map(|(hrs, cnt)| {
									VotingTrendItem { hrs: hrs as _, cnt: *cnt }
								})
								.collect::<Vec<_>>()
						} else {
							vec![]
						}
					}
				}
			})
			.collect::<Vec<_>>();
		ctx.paper_result.insert_many(cached_entries, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	}

	Ok(
		QueryQuestionnaireResponse {
			entries: result_entries
		}
	)
}

pub async fn papers_trend(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32, question_id: String) ->  Result<models::TrendResponse, ServiceError> {
	let (filter, cache_key) = process_query(query)?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year,
		"entry.question_id": {
			"$in": vec![question_id]
		}
	};
	let cached_entry = ctx.paper_result.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	if let Some(cached_entry) = cached_entry {
		let resp = models::TrendResponse {
			trend: cached_entry.trend,
			trend_first: vec![]
		};
		Ok(resp)
	} else {
		let resp = models::TrendResponse {
			trend: vec![],
			trend_first: vec![]
		};
		Ok(resp)
	}
}

pub async fn chars_covote(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32, top_k: i32) ->  Result<models::CovoteResponse, ServiceError> {
	let (filter, cache_key) = process_query(query.clone())?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};

	// step 1: query cache
	let lockid = format!("lock-chars_covote-{}", cache_key);
	let guard = ctx.lock.acquire_async(lockid.as_bytes(), 60 * 1000).await;
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year,
		"first_k": top_k
	};
	if let Some(cached) = ctx.covote_chars.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))? {
		return Ok(models::CovoteResponse { items: cached.items });
	}
	// step 2: build covote cache
	let mut rank_resp = chars_ranking(ctx, query, vote_start, vote_year).await?;
	rank_resp.entries.sort_by(
		|a, b| {
			a.rank.cmp(&b.rank)
		}
	);
	let top_k = std::cmp::min(rank_resp.entries.len(), top_k as usize);
	let mut contingency_tables = vec![[[0i32; 2]; 2]; ((top_k * top_k) - top_k) / 2 + top_k];
	let mut item2rank_map: HashMap<String, i32> = HashMap::with_capacity(top_k);
	let mut rank2item_map: Vec<String> = Vec::with_capacity(top_k);
	for rank_item in rank_resp.entries[..top_k].iter() {
		item2rank_map.insert(rank_item.name.clone(), rank_item.rank - 1);
		rank2item_map.push(rank_item.name.clone());
	}
	let mut total_votes = 0i32;
	let mut votes_cursor = ctx.votes_coll.find(filter, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	while let Some(Ok(vote)) = votes_cursor.next().await {
		let pv: PartialVote = bson::from_document(vote).unwrap();
		if let Some(items) = pv.chars {
			total_votes += 1;
			let item_ids = items
				.iter()
				.filter(|f| {
					item2rank_map.contains_key(&f.name)
				})
				.map(|f| {
					*item2rank_map.get(&f.name).unwrap()
				}).collect::<Vec<_>>();
			for id in item_ids.iter() {
				let rank_a = *id;
				for rank_b in 0..rank_a {
					let idx = (1 + rank_a) * rank_a / 2 + rank_b;
					let contingency_table = &mut contingency_tables[idx as usize];
					contingency_table[1][0] += 1;
					contingency_table[1][1] -= 1;
				}
				let rank_b = *id;
				for rank_a in (rank_b + 1)..(top_k as i32) {
					let idx = (1 + rank_a) * rank_a / 2 + rank_b;
					let contingency_table = &mut contingency_tables[idx as usize];
					contingency_table[0][1] += 1;
					contingency_table[1][1] -= 1;
				}
			}
			for item_comb in item_ids.iter().combinations(2) {
				let rank_a;
				let rank_b;
				if item_comb[0] >= item_comb[1] {
					rank_a = item_comb[0];
					rank_b = item_comb[1];
				} else {
					rank_a = item_comb[1];
					rank_b = item_comb[0];
				}
				let idx = (1 + rank_a) * rank_a / 2 + rank_b;
				let contingency_table = &mut contingency_tables[idx as usize];
				contingency_table[0][0] += 1;
				contingency_table[1][1] += 1;
				contingency_table[0][1] -= 1;
				contingency_table[1][0] -= 1;
			}
		}
	}
	for item in contingency_tables.iter_mut() {
		item[1][1] += total_votes;
	}
	let mut result = Vec::with_capacity(contingency_tables.len());
	for rank_a in 0..(top_k as i32) {
		for rank_b in 0..rank_a {
			let idx = (1 + rank_a) * rank_a / 2 + rank_b;
			let contingency_table = &mut contingency_tables[idx as usize];
			let m00 = contingency_table[0][0] as f64;
			let m01 = contingency_table[0][1] as f64;
			let m10 = contingency_table[1][0] as f64;
			let m11 = contingency_table[1][1] as f64;
			let total_votes = total_votes as f64;
			result.push(
				CovoteItem {
					a: rank2item_map[rank_a as usize].clone(),
					b: rank2item_map[rank_b as usize].clone(),
					cs: {
						(total_votes * (m00 * m11 - m01 * m10) * (m00 * m11 - m01 * m10)) /
						((m00 + m10) * (m00 + m01) * (m11 + m10) * (m11 * m01))
					},
					cv: {
						m00 / (m00 + m01 + m10)
					},
					mi: {
						(m00 * total_votes) / ((m00 + m01) * (m00 + m10))
					},
					m00: m00 as i32,
					m01: m01 as i32,
					m10: m10 as i32,
					m11: m11 as i32,
				}
			)
		}
	};
	let cached = CachedCovote {
		key: cache_key,
		vote_year: vote_year,
		first_k: top_k as i32,
		items: result.clone()
	};
	ctx.covote_chars.insert_one(cached, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	Ok(models::CovoteResponse { items: result })
}

pub async fn musics_covote(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime, vote_year: i32, top_k: i32) ->  Result<models::CovoteResponse, ServiceError> {
	let (filter, cache_key) = process_query(query.clone())?;
	let filter = if let Some(filter) = filter {
		doc! {
			"$and": [filter, {"vote_year": vote_year}]
		}
	} else {
		doc! {
			"vote_year": vote_year
		}
	};

	// step 1: query cache
	let lockid = format!("lock-musics_covote-{}", cache_key);
	let guard = ctx.lock.acquire_async(lockid.as_bytes(), 60 * 1000).await;
	// find in cache
	let cache_query = doc! {
		"key": cache_key.clone(),
		"vote_year": vote_year,
		"first_k": top_k
	};
	if let Some(cached) = ctx.covote_musics.find_one(cache_query.clone(), None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))? {
		return Ok(models::CovoteResponse { items: cached.items });
	}
	// step 2: build covote cache
	let mut rank_resp = musics_ranking(ctx, query, vote_start, vote_year).await?;
	rank_resp.entries.sort_by(
		|a, b| {
			a.rank.cmp(&b.rank)
		}
	);
	let top_k = std::cmp::min(rank_resp.entries.len(), top_k as usize);
	let mut contingency_tables = vec![[[0i32; 2]; 2]; ((top_k * top_k) - top_k) / 2 + top_k];
	let mut item2rank_map: HashMap<String, i32> = HashMap::with_capacity(top_k);
	let mut rank2item_map: Vec<String> = Vec::with_capacity(top_k);
	for rank_item in rank_resp.entries[..top_k].iter() {
		item2rank_map.insert(rank_item.name.clone(), rank_item.rank - 1);
		rank2item_map.push(rank_item.name.clone());
	}
	let mut total_votes = 0i32;
	let mut votes_cursor = ctx.votes_coll.find(filter, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	while let Some(Ok(vote)) = votes_cursor.next().await {
		let pv: PartialVote = bson::from_document(vote).unwrap();
		if let Some(items) = pv.musics {
			total_votes += 1;
			let item_ids = items
				.iter()
				.filter(|f| {
					item2rank_map.contains_key(&f.name)
				})
				.map(|f| {
					*item2rank_map.get(&f.name).unwrap()
				}).collect::<Vec<_>>();
			for id in item_ids.iter() {
				let rank_a = *id;
				for rank_b in 0..rank_a {
					let idx = (1 + rank_a) * rank_a / 2 + rank_b;
					let contingency_table = &mut contingency_tables[idx as usize];
					contingency_table[1][0] += 1;
					contingency_table[1][1] -= 1;
				}
				let rank_b = *id;
				for rank_a in (rank_b + 1)..(top_k as i32) {
					let idx = (1 + rank_a) * rank_a / 2 + rank_b;
					let contingency_table = &mut contingency_tables[idx as usize];
					contingency_table[0][1] += 1;
					contingency_table[1][1] -= 1;
				}
			}
			for item_comb in item_ids.iter().combinations(2) {
				let rank_a;
				let rank_b;
				if item_comb[0] >= item_comb[1] {
					rank_a = item_comb[0];
					rank_b = item_comb[1];
				} else {
					rank_a = item_comb[1];
					rank_b = item_comb[0];
				}
				let idx = (1 + rank_a) * rank_a / 2 + rank_b;
				let contingency_table = &mut contingency_tables[idx as usize];
				contingency_table[0][0] += 1;
				contingency_table[1][1] += 1;
				contingency_table[0][1] -= 1;
				contingency_table[1][0] -= 1;
			}
		}
	}
	for item in contingency_tables.iter_mut() {
		item[1][1] += total_votes;
	}
	let mut result = Vec::with_capacity(contingency_tables.len());
	for rank_a in 0..(top_k as i32) {
		for rank_b in 0..rank_a {
			let idx = (1 + rank_a) * rank_a / 2 + rank_b;
			let contingency_table = &mut contingency_tables[idx as usize];
			let m00 = contingency_table[0][0] as f64;
			let m01 = contingency_table[0][1] as f64;
			let m10 = contingency_table[1][0] as f64;
			let m11 = contingency_table[1][1] as f64;
			let total_votes = total_votes as f64;
			result.push(
				CovoteItem {
					a: rank2item_map[rank_a as usize].clone(),
					b: rank2item_map[rank_b as usize].clone(),
					cs: {
						(total_votes * (m00 * m11 - m01 * m10) * (m00 * m11 - m01 * m10)) /
						((m00 + m10) * (m00 + m01) * (m11 + m10) * (m11 * m01))
					},
					cv: {
						m00 / (m00 + m01 + m10)
					},
					mi: {
						(m00 * total_votes) / ((m00 + m01) * (m00 + m10))
					},
					m00: m00 as i32,
					m01: m01 as i32,
					m10: m10 as i32,
					m11: m11 as i32,
				}
			)
		}
	};
	let cached = CachedCovote {
		key: cache_key,
		vote_year: vote_year,
		first_k: top_k as i32,
		items: result.clone()
	};
	ctx.covote_musics.insert_one(cached, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	Ok(models::CovoteResponse { items: result })
}
