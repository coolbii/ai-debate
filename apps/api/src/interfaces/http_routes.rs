pub fn session_routes() -> Vec<&'static str> {
    vec![
        "POST /sessions",
        "GET /sessions/:id",
        "POST /sessions/:id/start",
        "POST /sessions/:id/pause",
        "POST /sessions/:id/resume",
        "POST /sessions/:id/next-turn",
        "POST /sessions/:id/declare-winner",
    ]
}
