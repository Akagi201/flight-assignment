use std::time::Instant;

use crate::models::*;
use crate::solution::find_total_path;
use axum::{
  http::StatusCode,
  response::IntoResponse,
  Json,
};

// basic health handler
pub async fn check_health() -> &'static str {
  "OK"
}

// solution handler
pub async fn check_solution(
  Json(req): Json<SolutionReq>,
) -> impl IntoResponse {
  tracing::debug!("input {:?}", &req.flights);
  let now = Instant::now();
  let res = find_total_path(req.flights);
  let elapsed = now.elapsed();
  tracing::debug!("output {:?}", &res);

  (StatusCode::CREATED, Json(SolutionResp {
      code: 0,
      message: format!("Elapsed: {:.2?}", elapsed),
      answer: res,
  }))
}

// global 404 handler
pub async fn handler_404() -> impl IntoResponse {
  (StatusCode::NOT_FOUND, "Wrong path, Post /solution")
}
