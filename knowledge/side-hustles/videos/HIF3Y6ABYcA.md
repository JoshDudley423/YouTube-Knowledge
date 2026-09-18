# AI Told Me to Start This Business. So I Did.
Channel: The Koerner Office | https://www.youtube.com/watch?v=HIF3Y6ABYcA | Published: 2025-08-19

Chris Koerner solo episode: a live, real-time walkthrough of his idea-validation process, using "build a QuickBooks/Xero competitor" as the worked example. Ends with him actually launching a landing page for a real product (LazyBooks). Note: Koerner's stated track record, self-reported — ~80 businesses started over 16 years, 3 worth $10M+, 7 worth $1M+.

## The business idea being validated: a QuickBooks/Xero competitor ("LazyBooks")
- Target market: businesses with 0-10 employees specifically (cited stat: ~35 million total US businesses, ~30 million of which have zero employees) — deliberately ignoring mid-market/enterprise.
- Differentiation hypothesis going in: AI-enabled, cheaper ($19/month), simpler with fewer/core features only, and — critically — reliable bank-feed connections that "never break" (a top complaint about incumbents).
- Planned feature: a toggle to switch language/terminology between "accountant mode" and "human mode" (e.g., hiding jargon like "chart of accounts" for the business owner while still giving the accountant what they need).
- Domain: lazybooks.com, purchased in advance (~$800) before doing the validation research shown in the video — i.e., Chris was already leaning toward launching and used this process to confirm/inform, not to gate a go/no-go decision from zero.

## Validation research findings (the actual market data uncovered)
- Via Reddit-analysis tool (GummySearch) across 7 small-business-related subreddits (~10.4M combined subscribers: r/smallbusiness, r/smallbusinesscanada, r/business, r/Entrepreneur, r/smallbusinessowners, r/businessideas, r/sideproject):
  - Top QuickBooks discussion patterns: "seeking alternatives to QuickBooks" (most common), concerns about pricing/subscription costs, integration issues, people building/considering alternatives, payroll/bookkeeping use, general frustrations.
  - Top Xero discussion patterns: integration issues, general "looking for accounting software recommendations" (people organically recommending Xero — read as a net positive signal for Xero).
  - Sentiment scores (0-100 scale): QuickBooks = 38/100; Xero = 48/100 — both negative-leaning, but QuickBooks worse.
  - Cross-tool "ask AI" query across the scraped posts revealed the #1 shared complaint for both products isn't just that pricing is high, but that pricing keeps increasing unpredictably — this directly shaped the proposed marketing message: "cheap, forever" (the explicit opposite of competitors' actual behavior).
- Via a multi-AI-tool prompt run in parallel across ChatGPT-5 (agent mode), Claude Opus 4.1 (web search + deep research), Perplexity Labs, and Manus (agentic, free plan) — all given the same detailed research prompt (asking them to read reviews from Capterra, G2/GetApp, Software Advice, TrustRadius):
  - Top pain points (consistent across tools): unreliable/breaking bank feed connections, unpredictable price increases, poor customer support, limited report customization, payroll limitations.
  - Top "must-have" features: cloud accessibility + multi-user collaboration (non-negotiable — sharing access with accountant/spouse/employees), ease of use/clean interface, invoicing + recurring billing, bank reconciliation, document attachments/paperless workflow, security.
  - Claude specifically flagged "payment processing disasters" as a distinct pain point — implying users resent being locked into a single in-house payment processor (i.e., a "no payment-processor hostage situation" positioning opportunity).
  - Suggested "wedge" opportunities to lead with: bank feeds that never break (reliability guarantee), freedom to choose your own payment processor, same-day human support, permanently transparent pricing, laser focus on small businesses only (not enterprise).
  - Overall verdict (paraphrased from the AI tools): the accounting-software space is crowded/mature, but user dissatisfaction is high enough to support a lean, customer-focused challenger; recommended approach is to prove traction in a niche vertical first (e.g., project-based service firms or nonprofits) before broadening. Customer acquisition cost, bank-integration engineering complexity, and feature-creep pressure over time were flagged as real risks, but the space was judged achievable for a technically capable, operationally disciplined small team (with "a 10x developer") to capture meaningful market share within roughly 2-3 years.
- General framing point made explicitly: a "commodity" market (something nearly everyone needs, like bookkeeping software) is not inherently bad — it can mean a very large addressable market rather than doomed price competition.

## The reusable validation process/playbook (the actual "how-to," most broadly useful part of this video)
1. **AI research tools used**: ChatGPT (agent mode), Claude (Opus 4.1, web search + deep research enabled), Perplexity (using "Labs" mode, not "Deep Research" — Labs was preferred because it auto-renders findings as charts/graphs/tables while still doing deep research), and Manus (agentic mode, free plan) — run the exact same detailed prompt across all four in parallel to cross-check/cluster consistent findings and reduce single-model bias. Full prompt text is provided in the video's own description/show notes.
2. When Claude asked clarifying follow-up questions (target market segment, differentiation hypothesis, focus features), Chris answered them live — reinforcing that a good validation prompt should specify target customer segment (e.g., "0-10 employee businesses") and your working differentiation thesis (price, UX, vertical focus) rather than staying fully generic.
3. **Non-AI research tools used**:
   - **GummySearch** (built on top of Reddit): create a custom "audience" of relevant subreddits, then search a keyword (e.g., "QuickBooks") and use its "Patterns" tab to auto-summarize repeating themes across hundreds/thousands of posts, its "Sentiment" tab to score positive/negative sentiment, and its "Ask" feature to query the aggregated post data directly with a natural-language question. Cost: ~$10/day pass, or $29-$200/month for ongoing plans (Chris just bought a day pass for this one-off research session).
   - **PickFu**: a paid-survey/polling platform with a large panel of pre-profiled respondents (nationality, age, gender, income, interests, spending habits, etc.) — useful for getting a shortcut audience when you don't have your own. You choose sample size (recommends doubling the default 50 to 100 respondents for reliability) and pay per response (~$1/response, so ~$100 for 100 targeted responses, e.g., US small business owners); results typically arrive within about an hour. More specific audience targeting costs more.
   - **Reddit / Facebook groups directly**: manually search relevant subreddits or well-moderated (non-spammy) Facebook groups for a keyword (e.g., "QuickBooks"), then paste the resulting thread URLs into ChatGPT to have it summarize the findings — a free/manual alternative to GummySearch.
4. **Speed-to-landing-page workflow** (demonstrated live, timed at 8 minutes total): use Carrd.co (~$50/year for up to 25 sites) to duplicate an existing landing page template, rename it, connect a custom domain purchased separately (via Namecheap/GoDaddy/Cloudflare — walkthrough shows adding A and CNAME DNS records), and wire the page's email capture form to an email platform (he uses Beehiiv; Mailchimp/Klaviyo are viable alternatives) via API key + publication ID, with UTM parameter tracking enabled so signups can be segmented by source landing page. Total build cost cited: "about $2/year the way I do it" (excluding the cost of the custom domain itself, which is optional).
5. Framing/philosophy: the entire point of this process is to gather real signal on demand, competitive weaknesses, and must-have features BEFORE spending money or committing to building the actual product — explicitly aimed at people without an existing audience or budget, since most of these tools have free trials or low-cost one-off usage options.

## Tools/platforms named
- ChatGPT (GPT-5, agent mode), Claude (Opus 4.1, web search + deep research), Perplexity (Labs mode), Manus (agentic mode) — AI research tools
- GummySearch (Reddit-based sentiment/pattern research tool)
- PickFu (paid audience polling platform)
- Reddit, Facebook groups (manual community research)
- Capterra, G2/GetApp, Software Advice, TrustRadius (review sites the AI tools were directed to pull data from)
- Carrd.co (landing page builder)
- Beehiiv (email list/newsletter platform used for capture; Mailchimp/Klaviyo noted as alternatives)
- Namecheap (domain registrar/DNS management shown; GoDaddy/Cloudflare noted as equivalents)
- Outscraper, Apify, ScrapeBox (mentioned in passing as other general web-scraping tool options, not used directly in this video)

## Caveats / risks
- Chris explicitly acknowledges he'd already bought the lazybooks.com domain (~$800) before running this validation, "being optimistic that this research would show me what I thought it would show" — i.e., this is presented as a real validation methodology, but in this instance it was somewhat confirmatory rather than a blind go/no-go filter.
- Both ChatGPT and Perplexity's own outputs concede the accounting-software market is "crowded and mature" — the opportunity is framed as real but requires disciplined execution on a narrow set of pain points, not a novel/uncontested space.
- Tools flagged as needing specific, well-targeted prompts and audience selection to be useful — generic prompts across AI tools were expected to produce "a lot of redundant responses," so the value comes from precision in what you ask and which sources you direct the AI to use.
- Success in this space (per Perplexity's own caveat) requires real technical execution ability — reliable bank-feed integration engineering is called out specifically as "significant" complexity, not a trivial no-code build.
