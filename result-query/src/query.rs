use std::collections::{HashMap, HashSet};

use bson::Document;
use chrono::Utc;
use mongodb::Collection;
use futures::stream::{StreamExt, TryStreamExt};
use serde_derive::{Serialize, Deserialize};


use crate::{parser, common::SERVICE_NAME, context::AppContext, models::{self, SubmitMetadata, RankingEntry, VotingTrendItem, RankingQueryResponse}, service_error::ServiceError};

#[derive(Clone, Serialize, Deserialize)]
struct PartialVotePaperEntry {
	pub opt: Vec<String>
}

#[derive(Clone, Serialize, Deserialize)]
struct PartialVote {
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub chars: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub chars_first: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub musics: Option<Vec<String>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	#[serde(default)]
	pub musics_first: Option<Vec<String>>,
	pub q11011: PartialVotePaperEntry,
	pub chars_meta: SubmitMetadata,
	pub musics_meta: SubmitMetadata,
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

pub async fn chars_ranking(ctx: &AppContext, query: Option<String>, vote_start: bson::DateTime) ->  Result<models::RankingQueryResponse, ServiceError> {
	let (filter, cache_key) = process_query(query)?;
	// find in cache
	// else
	let mut votes_cursor = ctx.votes_coll.find(filter, None).await.map_err(|e| ServiceError::new(SERVICE_NAME, format!("{:?}", e)))?;
	let mut hrs_bins: HashMap<String, Vec<i32>> = HashMap::new();
	let mut per_char_vote_count: HashMap<String, i32> = HashMap::new();
	let mut per_char_vote_first_count: HashMap<String, i32> = HashMap::new();
	let mut per_char_male_vote_count: HashMap<String, i32> = HashMap::new();
	let mut per_char_female_vote_count: HashMap<String, i32> = HashMap::new();
	let mut total_votes = 0i32;
	let mut total_first_votes = 0i32;
	let mut total_male = 0i32;
	let mut total_female = 0i32;
	while let Some(Ok(vote)) = votes_cursor.next().await {
		let pv: PartialVote = bson::from_document(vote).unwrap();
		if pv.chars.is_none() {
			continue;
		}
		if let Some(f) = &pv.chars_first {
			if f.len() != 0 {
				total_first_votes += 1;
				*per_char_vote_first_count.entry(f[0].clone()).or_default() += 1;
			}
		}
		let chs = pv.chars.as_ref().unwrap();
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
		let hrs_diff = (pv.chars_meta.created_at.to_chrono() - vote_start.to_chrono()).num_hours() as usize;
		for ch in chs {
			*per_char_vote_count.entry(ch.clone()).or_default() += 1;
			if !hrs_bins.contains_key(ch) {
				hrs_bins.insert(ch.clone(), vec![0i32; 24 * 30]);
			}
			let trend_hrs_bins = hrs_bins.get_mut(ch).unwrap();
			trend_hrs_bins[hrs_diff] += 1;
			if is_male {
				*per_char_male_vote_count.entry(ch.clone()).or_default() += 1;
			} else {
				*per_char_female_vote_count.entry(ch.clone()).or_default() += 1;
			}
		}
	}
	let mut chars_result = Vec::with_capacity(300);
	let mut per_char_vote_count_vec: Vec<(&String, &i32)> = per_char_vote_count.iter().collect();
	let mut per_char_vote_count_count_only_vec: Vec<i32> = per_char_vote_count.iter().map(|(a, b)| *b).collect();
	per_char_vote_count_count_only_vec.sort();
	per_char_vote_count_vec.sort_by(|a, b| b.1.cmp(a.1));
	let mut rank = 1;
	for (ch, _) in per_char_vote_count_vec {
		let trend = hrs_bins
			.get(ch)
			.unwrap()
			.iter()
			.enumerate()
			.map(|(hrs, cnt)| {
				VotingTrendItem { hrs: hrs as _, vote_count: *cnt }
			})
			.collect::<Vec<_>>();
		let entry = RankingEntry {
			rank: rank,
			name: ch.clone(),
			vote_count: *per_char_vote_count.get(ch).unwrap(),
			first_vote_count: *per_char_vote_first_count.get(ch).unwrap(),
			first_vote_percentage: *per_char_vote_first_count.get(ch).unwrap() as f64 / *per_char_vote_count.get(ch).unwrap() as f64,
			first_vote_count_weighted: per_char_vote_count.get(ch).unwrap() + per_char_vote_first_count.get(ch).unwrap(),
			character_type: "todo".to_owned(),
			character_origin: "todo".to_owned(),
			first_appearance: "todo".to_owned(),
			name_jpn: "todo".to_owned(),
			vote_percentage: *per_char_vote_count.get(ch).unwrap() as f64 / total_votes as f64,
			first_percentage: *per_char_vote_first_count.get(ch).unwrap() as f64 / total_first_votes as f64,
			male_vote_count: *per_char_male_vote_count.get(ch).unwrap(),
			male_percentage_per_char: *per_char_male_vote_count.get(ch).unwrap() as f64 / *per_char_vote_count.get(ch).unwrap() as f64,
			male_percentage_per_total: *per_char_male_vote_count.get(ch).unwrap() as f64 / total_male as f64,
			female_vote_count: *per_char_female_vote_count.get(ch).unwrap(),
			female_percentage_per_char: *per_char_female_vote_count.get(ch).unwrap() as f64 / *per_char_vote_count.get(ch).unwrap() as f64,
			female_percentage_per_total: *per_char_female_vote_count.get(ch).unwrap() as f64 / total_female as f64,
			trend,
		};
		chars_result.push(entry);
		rank += 1;
	};
	let num_char = per_char_vote_count_count_only_vec.len();
	let avg = total_votes as f64 / num_char as f64;
	let median = if num_char % 2 == 0 {
		0.5f64 * (
			per_char_vote_count_count_only_vec[num_char / 2 - 1] as f64 +
			per_char_vote_count_count_only_vec[num_char / 2] as f64
		)
	} else {
		per_char_vote_count_count_only_vec[num_char / 2] as f64
	};
	let resp = RankingQueryResponse {
		entries: chars_result,
		total_unique_items: num_char as _,
		total_first: total_first_votes,
		total_votes,
		average_votes_per_item: avg,
		median_votes_per_item: median,
	};
	Ok(resp)
}
