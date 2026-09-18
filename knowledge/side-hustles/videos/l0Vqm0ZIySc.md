# He Asked AI To Make Money. It Did.
Channel: The Koerner Office | https://www.youtube.com/watch?v=l0Vqm0ZIySc | Published: 2026-03-20

Interview with Robbie (non-technical, works in operations at a day job), who gave an AI agent tool called "OpenClaw" (connected to Claude Code, nicknamed "Ron" by Robbie) a $100 budget and a goal to build a business, documenting the process on TikTok.

## The business idea
- Not a predefined idea — the actual business ("AI Co-Founder Club" / selling access to a hosted personal AI agent) emerged from a chain of pivots the AI agent itself proposed, with Robbie acting purely as "human in the loop" (clicking buttons, providing credentials, approving actions) rather than directing strategy.
- Origin/inspiration: referenced a 2023 "HustleGPT" experiment by Jackson Greathouse Falls (gave GPT-4 $100 and told it to make money via Twitter), which Robbie adapted using OpenClaw roughly 2.5 years later.
- Final product: sells paying customers ($29/month) their own hosted AI agent instance (run on rented bare-metal servers, containerized/sandboxed for safety) plus community access (Discord, templates, peer learning) — essentially "AI-agent-as-a-service" for non-technical people who want to experiment with agentic AI without hosting it themselves or exposing their own machine to risk.

## Numbers
- Starting budget given to the AI: $100 (originally $200 in the original TikTok video, described as covering a Claude Max subscription).
- Pre-launch validation: posted the concept on TikTok with a Stan Store link requiring a $10 deposit to "pre-order" (filters out non-serious "tire kickers") — got 617 pre-orders within about 2 weeks, i.e., roughly $6,000 collected before the product was built.
- Conversion from $10 depositors to paying $29/month members: 270 out of 600 people (~40-45% close rate).
- Revenue after 13 days of being open: $8,374 MRR (~$100K annualized run rate), ~$6,000/month net profit at that point.
- Costs: ~$600/month for four bare-metal dedicated servers rented from Contabo (~$150/unit); ~$2,000/month for LLM inference/token costs — combined cost of goods sold roughly $2,500/month.
- One user reportedly spawned 125 sub-agents in 5 days and burned 1.3 million tokens in 48 hours, prompting Robbie to plan a free-token cap with a cheaper fallback model or a "bring your own API key" option for heavy users.
- Marketing spend: $0 — entirely organic TikTok posting, no paid ads, no cold outreach.

## Process / steps
- Gave the AI agent an open-ended prompt/conversation (not a specific business plan) about what kind of business it thought it could run; the agent itself proposed offering SWOT/market-analysis research services on Fiverr.
- First attempt (Fiverr SWOT-analysis freelance gigs) failed — a brand-new Fiverr account with no reviews/visibility got no traction, despite Robbie noting there appeared to be real demand for that kind of research service on Fiverr.
- Pivoted: had the agent analyze his own TikTok performance and comments (using Apify, a comment-scraping tool) for feedback; that video's ~200 comments included "I want a Ron / how do I get this" style requests. The agent itself flagged this cluster of demand as the real opportunity, and Robbie/agent pivoted the whole business around productizing "your own agent" rather than continuing the original idea.
- Built the actual delivery infrastructure by having the (non-technical) human simply follow the agent's own technical instructions: rent bare-metal servers from Contabo, run OpenClaw inside a Docker container per customer so each instance is sandboxed (the agent only knows what the user explicitly tells it — no access to email, credit cards, files, etc. unless granted).
- Ran the entire pre-launch/launch purely through organic TikTok videos featuring the AI "Ron" character narrating its own progress (including AI-generated video scripts), driving viewers to a Stan Store landing page.
- Used a $10 deposit-gated "pre-order" mechanic (via Stan Store) before any product existed, to validate real intent to pay before building.
- Post-launch: went "heads down" for the first ~10 days to stabilize the product/experience before resuming public posting and marketing.
- Interaction channels for customers to talk to their agent: Discord, Telegram, or the built-in UI. A browser extension to let the agent access a customer's own computer/files was in development but being rolled out cautiously due to safety concerns.
- Planned next step/upsell: a smaller, more serious cohort-style offering aimed at people who want to turn their agent into a genuine "AI employee" for business use (vs. current users who mostly use it for art, casual chat, or personal-assistant tasks).

## Tools / platforms named
- OpenClaw (the AI agent framework, connects to Claude Code) — core product.
- Contabo — cheap bare-metal dedicated server rental (~$150/server/month).
- Docker — used to containerize/sandbox each customer's agent instance for safety.
- Apify — comment-scraping tool used to pull TikTok comment data.
- TikTok — sole marketing/distribution channel, entirely organic.
- Stan Store — used for the $10 deposit-gated pre-order landing page.
- Fiverr — abandoned first attempt at monetization (AI-run SWOT analysis freelance gigs).
- Discord — community/support channel for paying members.

## Caveats / risks
- OpenClaw running directly on a user's own local machine carries real risk — cited horror stories of it deleting full email inboxes and one case of an agent deleting a 2-year GitHub repository; this is exactly why the business containerizes/sandboxes each instance rather than giving raw local access.
- Token/inference costs can spike unpredictably with heavy users (one user burned 1.3M tokens in 48 hours) — the business had not yet solved usage-based cost control at the time of the interview (planned fixes: usage caps, cheaper fallback models, bring-your-own-API-key option).
- Both guest and host frame this as an extremely early-stage technology/market ("OpenClaw is about 100 days old," compared to being present "100 days into the internet" or "2007 social media") — meaning both large opportunity and correspondingly high uncertainty/immaturity of the tooling and safety practices.
- Business legitimacy caveat baked into the video's own disclaimer: presented for educational/entertainment purposes, not guaranteed to be replicable, no guarantee of similar earnings.
- Broader framing point made in the episode: most business owners are not using AI tools as heavily as the agent-economy assumes, so market opportunities others might dismiss as "already saturated by AI users" are often still wide open.
