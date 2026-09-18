# I Asked AI for a Money-Making Website (It Built One in Minutes)
Channel: The Koerner Office | https://www.youtube.com/watch?v=CKiszDu2aic | Published: 2026-07-24

Solo, sponsored (Abacus.AI / ChatLLM) episode: builds a directory website from scratch live, using AI to both choose the niche and build the site.

## The business idea: directory websites
- Directory sites (niche business listing sites) are framed as a durable, low-maintenance business model that predates AI and will persist through it — because AI search/agents need well-organized, trustworthy sources to pull from.
- Appeal: once built, largely passive/self-running.
- Three monetization paths for a directory:
  1. **Featured/paid placement upgrades** — list every relevant business for free without asking permission (framed as doing them a favor), charge only for a higher/featured position, generally attractive to a business once it's already seeing real leads/traffic from the site.
  2. **Banner ads** — described as the lowest-effort, most passive monetization but also the "lowest and worst use" of the site's attention/traffic.
  3. **Lead reselling** — collect consumer inquiries directly (e.g., "I need an estate cleanout") and sell/auction each lead to the directory's listed businesses (e.g., email all of them simultaneously: "here's a lead, this neighborhood, this estimated value, who wants it?").
- Concrete pricing example built into the demo site: a $30/month "featured listing" upgrade via Stripe.

## Process used to pick and validate the niche
1. Used ChatLLM by Abacus.AI (a multi-model chat/agent platform) to query multiple AI models with the same prompt: "What's an underserved local directory niche I could build and monetize? Give me one specific niche, who pays for it, and how it makes money."
2. Got different answers from different models (used as generic examples of the exercise, not verified real market data): commercial kitchen equipment repair (B2B), estate cleanout/senior downsizing, aging-in-place/accessibility home modification (consumer).
3. Framing for B2B vs. consumer directories: B2B niches generally carry higher per-lead/deal value but are harder to break into (must reach a decision-maker); consumer niches make less per lead but are easier to convert (no gatekeeper, direct-to-payer).
4. Chose the "estate cleanout" niche partly from personal bias (host has a friend actively doing an adjacent business: buying leftover items from estate sales cheaply — e.g., paying ~$200 for everything left over — then reselling on Facebook Marketplace) and cited macro tailwind: ~10,000 baby boomers retiring/aging out daily, creating ongoing demand from their (often millennial) children needing to liquidate parents' estates.
5. Validated demand using Google Trends: compared search terms ("estate sale," "commercial kitchen equipment repair," "aging in place," "equipment repair," "how to host an estate sale," "estate cleanout") — "estate cleanout" showed the strongest/most consistently rising trend line of the terms tested, reinforcing the niche choice.
6. Positioning/naming insight: naming the business "Estate Cleanout Pros" rather than anything resembling "junk removal" matters — people don't perceive their late relative's possessions as "junk" and may find a junk-removal framing disrespectful, so brand language should target the emotional context of the buyer.
7. Side note (tangential business observation, not the directory idea itself): an estate-sale-cleanout service (vs. generic junk removal) tends to receive higher-value resellable items as part of the job, creating a "negative COGS" dynamic — you get paid to take inventory you can then resell, unlike paying to source inventory normally.

## Building the directory site itself (mechanical/how-to steps)
1. Used the AI agent mode (vs. plain chat mode) in ChatLLM to do heavier data-generation work — cost cited as ~$10/month for agent access to many premium models combined (vs. $20-200/month each if subscribed individually).
2. Prompted the agent: "Compile 20 real businesses that do estate sale cleanouts in the greater Dallas-Fort Worth area. For each one, give the business name, category, phone, and a one-line description. Put it all in a table." Iterated the prompt after an initial wording mistake produced irrelevant results (illustrating garbage-in/garbage-out).
3. Cross-checked outputs across different underlying models (ChatGPT, Claude Opus, Fable 5, and a "Route LLM" auto-selector that picks the best model per prompt) and used a second, different model to fact-check the first model's output (phone numbers, addresses) before publishing.
4. Downloaded the resulting business list as CSV, then switched the AI tool into full agent mode with the prompt: "Build a directory website that lists estate sale cleanout businesses. Include a searchable home page, category pages, and individual listing pages with a photo, description, and contact info. Prefill it with the list I attached. Add a paid featured listing upgrade with Stripe so a business can pay $30/month to appear at the top."
5. Answered agent clarification questions: business name ("Estate Cleanout Pros"), allow business self-submission (yes), categories (let AI infer from data), payment confirmation email recipient (own email), color scheme (clean/minimal).
6. Connected payments by generating a Stripe secret/publishable API key pair (Stripe dashboard → API keys → create secret key) and pasting into the agent's setup flow.
7. Chose to launch nationwide rather than restricting to one metro (Dallas-Fort Worth), reasoning that a very niche industry should pair with broad geography (and vice versa — broad industries should pair with a narrow geography) to ensure sufficient search volume; can always spin off a geo-specific domain later if data shows concentrated regional demand.
8. To scale content: re-run the same business-compiling prompt asking explicitly for a much larger number of listings (host suggests requesting 1,000+) since models default to doing less work unless told to work harder ("token maxing" — deliberately using higher-capability models/more compute than strictly required to get thorough output and to learn each model's behavior).
9. Generated a custom hero image for the site using an AI image generator built into the same tool (host's preferred image model mentioned: GPT Image).
10. Deployed the finished site with one click to a subdomain hosted by the AI platform (Abacus); reviewed the live site (search bar, category filter buttons, individual listings, working Stripe checkout flow, mobile-responsive layout, and an admin panel showing listing counts/featured status).

## Caveats/risks noted
- AI-suggested niches are not verified market opportunities on their own — the host explicitly does additional validation (Google Trends comparison) before committing, and notes his own choice was influenced by personal bias (a friend's adjacent business) rather than data alone.
- Garbage-in/garbage-out: an imprecise prompt ("20 real estate cleanout businesses" vs. "20 real businesses that do estate sale cleanouts") produced irrelevant results (real-estate agents instead of estate-cleanout services) — prompt wording matters and outputs should be checked.
- AI-generated business directory data (names, phone numbers, addresses) should be fact-checked, ideally by a second, different AI model, before publishing live to avoid inaccurate listings.
- This episode is a live build demo, not a completed/monetized case study — no revenue results are reported since the site was only just launched by the video's end.

## Tools/platforms named
- ChatLLM by Abacus.AI — the multi-model chat + AI agent platform used for both idea generation and building/deploying the actual website (sponsor; ~$10/month, referral link in description).
- Underlying LLMs referenced/compared: ChatGPT, Claude (Opus), Gemini, Fable 5, DeepSeek, Grok, plus a "Route LLM" auto-model-selection feature.
- Google Trends — used to validate relative search demand across candidate niches/terms.
- Stripe — payment processing for the $30/month featured-listing upgrade.
