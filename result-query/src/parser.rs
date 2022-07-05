

use bson::{doc, Document};
use pest::{Parser, iterators::Pairs};

#[derive(Parser)]
#[grammar = "query.pest"] // relative to src
pub struct QueryParser;



pub struct Query {
	pub root: Document
}

fn parse_value(mut root: Pairs<Rule>) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
	let q = root.next().unwrap();
	match q.as_rule() {
		Rule::int_inner => {
			Ok(q.as_str().to_string())
		}
		Rule::string => {
			Ok(q.into_inner().next().unwrap().as_str().to_string())
		}
		_ => unreachable!()
	}
}

fn parse_value_list(mut root: Pairs<Rule>) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
	let mut ret = vec![];
	while let Some(v) = root.next() {
		ret.push(parse_value(v.into_inner())?)
	}
	Ok(ret)
}

fn parse_in_condition(mut root: Pairs<Rule>) -> Result<Document, Box<dyn std::error::Error + Send + Sync>> {
	let ident = root.next().unwrap().as_str();
	let value_list = root.next().unwrap();
	let mut vl = parse_value_list(value_list.into_inner())?;
	vl.sort();
	Ok(doc! {
		ident: {
			"$in": vl
		}
	})
}

fn parse_eq_condition(mut root: Pairs<Rule>) -> Result<Document, Box<dyn std::error::Error + Send + Sync>> {
	let ident = root.next().unwrap().as_str();
	let v = root.next().unwrap();
	Ok(doc! {
		ident: {
			"$in": [parse_value(v.into_inner())?]
		}
	})
}

fn parse_condition(mut root: Pairs<Rule>) -> Result<Document, Box<dyn std::error::Error + Send + Sync>> {
	let q = root.next().unwrap();
	match q.as_rule() {
		Rule::in_condition => {
			parse_in_condition(q.into_inner())
		},
		Rule::eq_condition => {
			parse_eq_condition(q.into_inner())
		},
		a => {
			unreachable!()
		}
	}
}

fn parse_primary_query(mut root: Pairs<Rule>) -> Result<Document, Box<dyn std::error::Error + Send + Sync>> {
	let q = root.next().unwrap();
	println!("{}",q);
	match q.as_rule() {
		Rule::query => {
			parse_query(q.into_inner())
		},
		Rule::condition => {
			parse_condition(q.into_inner())
		},
		Rule::primary_query => {
			parse_primary_query(q.into_inner())
		},
		x => {
			unreachable!()
		}
	}
}

fn parse_or_query(mut root: Pairs<Rule>) -> Result<Document, Box<dyn std::error::Error + Send + Sync>> {
	let left_and_query = parse_and_query(root.next().unwrap().into_inner())?;
	let right_query = parse_query(root.next().unwrap().into_inner())?;
	Ok(doc! {
		"$or": [left_and_query, right_query]
	})
}

fn parse_and_query(mut root: Pairs<Rule>) -> Result<Document, Box<dyn std::error::Error + Send + Sync>> {
	let primary_query = root.next().unwrap();
	let left_primary_query = parse_primary_query(primary_query.into_inner())?;
	if let Some(primary_query) = root.next() {
		let right_primary_query = parse_primary_query(primary_query.into_inner())?;
		Ok(doc! {
			"$and": [left_primary_query, right_primary_query]
		})
	} else {
		Ok(left_primary_query)
	}
}

fn parse_query(mut root: Pairs<Rule>) -> Result<Document, Box<dyn std::error::Error + Send + Sync>> {
	let q = root.next().unwrap();
	match q.as_rule() {
		Rule::or_query => {
			parse_or_query(q.into_inner())
		},
		Rule::and_query => {
			parse_and_query(q.into_inner())
		},
		_ => unreachable!()
	}
}

fn parse_root(mut root: Pairs<Rule>) -> Result<Document, Box<dyn std::error::Error + Send + Sync>> {
	let q = root.next().unwrap().into_inner().next().unwrap();
	match q.as_rule() {
		Rule::query => {
			parse_query(q.into_inner())
		},
		a => {
			println!("{:?}", a);
			unreachable!()
		}
	}
}

pub fn generate_mongodb_query(query: &str) -> Result<Document, Box<dyn std::error::Error + Send + Sync>> {
	let root = super::parser::QueryParser::parse(Rule::root, query)?;
	parse_root(root)
}

#[test]
pub fn test_parser_1() {
	let q = "(q11011=1101102 AND q11021=1102101) OR (chars:[\"博丽灵梦\",\"东风谷早苗\"] AND chars_first=\"东风谷早苗\")";
	let ret = generate_mongodb_query(q).unwrap();
	println!("{:#?}",ret);
	let ref_ret = doc! {

	};
	todo!()
}
