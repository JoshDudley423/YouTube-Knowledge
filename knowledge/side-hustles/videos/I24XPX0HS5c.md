# 6 Ways to Make Money With the New GPT Agent (It Blew My Mind)
Channel: The Koerner Office | https://www.youtube.com/watch?v=I24XPX0HS5c | Published: 2025-07-20

Demo of OpenAI's "ChatGPT Agent" (browser-controlling agent, $20/mo ChatGPT Plus) applied to real business tasks. Not a single business idea — a tour of how to use an AI agent to do the grunt work behind several side-hustle/agency offers.

## Business ideas / use cases demonstrated

- **Website-building for local businesses**: counter-intuitive targeting tip — pitch business owners who *already have* a website (statistically it's probably ugly/outdated), not ones with no website. People with no website have already been pitched constantly and don't value one; people who already invested in a website value having one and are easier to upsell/redo. Analogy given: a pressure-washing business owner throws promo flyers on the *cleanest* driveways (people who already value clean driveways), not the dirtiest ones.
- **Lead-gen + cold outreach as a service**: prompted agent to find 20 plumbers in Nashville with existing websites, cross-reference against directories like Manta (has "tens of millions of business listings") for owner emails/cell numbers, output to a Google Sheet/CSV. Took ~30 minutes and glitched repeatedly because Google Sheets login required staying on the tab; falling back to "just export as CSV" avoided the glitching. Rated 6.5/10 for that task.
- **Hyper-personalized cold email / SEO agency for a niche (e.g., dentists)**: prompt — find 5 dentist owners in Austin, research them (bio, hobbies, career history), write "almost creepily" personalized cold emails pitching local SEO services, and (optionally) send them directly from the user's Gmail. Full research + 5 personalized emails drafted in ~3 minutes. Rated 7.5/10 (glitched navigating Gmail because user kept switching tabs while it needed to stay on-screen).
- **Competitive research / consulting service**: pointed the agent at own e-commerce brand (texasnacks.com) and asked it to find the 5 biggest competitors by public traffic data and report what they sell well, SEO strength, and takeaways. Correctly identified Amazon/Walmart as top competitors (not an obvious answer) plus niche competitors the owner hadn't heard of. Framed as sellable: "you can use a prompt like this to do competitive research on other companies and then sell them that research... as an expert, consultant, or agency owner." Rated 8.5/10.
- **Client-meeting prep / research assistant service**: gave agent calendar access; it researched each upcoming meeting attendee (news, business updates) and produced a full briefing doc with talking points in 9 minutes — framed as replacing a human executive assistant. Rated 9/10.
- **Trend/opportunity research for picking a side hustle**: prompted "analyze Google Trends for five business ideas, check competition levels, estimate market size, rank by opportunity score, must be affordable to start" — returned pet sitting/dog walking, mobile car wash, home cleaning, online tutoring, social media marketing for local businesses, ranked by opportunity score (pet sitting/dog walking #1: fragmented market, 99% independent operators, moderate competition). Creator noted results were generic because he didn't feed it niche reference ideas.
- **Deep-dive market validation via scraped case studies**: for mobile car wash, prompted agent to scrape Reddit/Facebook/X/blogs/YouTube for real entrepreneur case studies with startup costs and early sales figures. Returned concrete numbers: one Reddit case — spent $811 startup, made $920 from 5 customers in month one; a cited YouTube case — 22-year-old built a $50k/month detailing business. Suggested next step: ask for 40 stories instead of 4, then have the agent synthesize a day-by-day 30-day launch plan from the best tactics across all of them.
- **Product idea validation via review-mining**: for a hypothetical "healthy powdered energy drink," had agent scrape Amazon reviews of similar products for common complaints, then generate a pie chart of complaint proportions (mixability/clumping 31%, taste issues 25%, packaging 12%, price 12%, limited flavor 6%, counterfeit concerns 6%) and a feature list addressing each pain point (easy-open tamper-evident packaging, anti-clumping formula, balanced sweetness, flavor variety, caffeine-free option, affordable price).
- **Investor pitch deck generation from research**: had the agent turn the energy-drink complaint research directly into an investor pitch deck; then improved it by uploading Airbnb's famous original pitch deck as a reference image/file and asking it to restyle the new deck in that format (coral accents, same slide structure) while incorporating personal biographical details it already knew about the user from chat history. First rough deck took ~12 minutes; refined 5-slide version came out well-designed. Rated 9.5/10 overall — called the standout demo of the video.

## Concrete numbers cited

- ChatGPT Plus subscription: $20/month gives access to Agent mode; creator ran 6 agents concurrently.
- Reddit case study: $811 spent to start a detailing side hustle, $920 revenue from 5 customers in month one (breakeven within one month).
- Cited YouTube case: 22-year-old running a $50k/month detailing business.
- Community upsell mentioned: Chris's own paid community "TK Owners" (tkowners.com) at $99/month.
- Energy drink market cited by the agent: $79 billion global market size.

## Process / prompting tips

- Give the agent concrete reference data/examples rather than vague asks — e.g., instead of "give me 10 approachable business ideas," say "give me 10 ... here are 4 examples: dog walking, car detailing, landscaping, farmers market stand" so it anchors on that pattern. Reference data (including reference images, in image-gen contexts) sharply improves output relevance.
- Explicitly name where to scrape (Reddit, specific subreddits, X, niche blogs, YouTube) — otherwise the agent defaults to generic sources like a CNBC article with no real tactics.
- Telling the agent to "move fast" measurably speeds up its work.
- Login/account access (Gmail, Google Sheets, Drive, calendar) is the main friction point: the agent often needs you to stay on that browser tab and not navigate away, or it glitches/loses context. Minimizing required logins makes runs faster and more reliable.
- Iterative follow-up works well once research is done — ask the agent case-specific analytical questions afterward (e.g., "which complaint appeared most often") rather than only requesting a one-shot dump, since the underlying LLM can reason over its own gathered data.

## Caveats / risks

- Task times were inconsistent and sometimes much longer than the UI's stated estimate (a "2 minute" task actually took 15-20 minutes).
- Agent lost session/browser control multiple times when the user tabbed away during logged-in tasks (Gmail, Google Sheets) — treated as a current UX limitation, not a fundamental blocker.
- Sending real cold emails via agent access to Gmail raises obvious liability/ethics questions if used for an actual, non-consented outreach campaign — the creator explicitly says he doesn't actually sell SEO services and sent the test emails anyway "to see if it'll work."
- Note of caution on relying on default agent research depth: initial trend-analysis output was "kind of generic" (not niche) because the user didn't supply enough example/reference ideas — output quality is prompt-dependent, not automatic.
- This is a fast-evolving tool category; capabilities and pricing described are a snapshot as of July 2025 and likely to change.
