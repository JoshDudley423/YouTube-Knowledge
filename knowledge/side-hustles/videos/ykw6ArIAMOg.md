# Stop Chasing Billion Dollar Ideas. Do This Instead.
Channel: The Koerner Office | https://www.youtube.com/watch?v=ykw6ArIAMOg | Published: 2025-04-07

Riff/brainstorm episode with Sam Thompson (recurring guest, @ImSamThompson). Part 1 of a two-part episode. Covers several concrete side-hustle/business ideas built around AI tools, automation, and digital products, plus tactics for viral content and low-cost testing.

## Idea 1: AI "GPT wrapper" apps (case study: InstantChef / sourdough app)
- Sam built a cooking-recipe GPT-wrapper app ("InstantChef"), planning to charge ~$60/year subscription.
- Used AI to write the entire app store/website copy (fed it a prompt like "give me 15 feature bullet points," pasted the output straight in) — needed a website only because Stripe requires one to set up an account.
- Website built via a purchased template plus mockups, not custom-coded; comparable custom sites elsewhere reportedly cost ~$112,000, versus the template-based approach he used from 72hoursites.com for $2,500.
- Go-to-market plan: run Facebook paid ads directly to the landing page, test that against running ads directly to the App Store listing, and separately test buying/running niche themed content pages in the cooking space to drive organic traffic.
- Framing/expectation: explicitly not expected to be a career-defining hit, but plausibly worth $100K-$200K — described as a low-risk "swing" (worst case lose the ~$X spent building it, best case a "software company printing cash").

## Idea 2: Automated viral-content "theme pages" (repost/aggregation accounts)
Process for turning other people's viral short-form videos into a scheduled content pipeline, largely without manual editing:
1. Use a Zapier Chrome extension: scroll a target niche hashtag (e.g., "vegan recipes") on Instagram desktop, click "add to spreadsheet" via Zapier on each video you want — creates a new row/link per video. Can grab ~60 videos in about 10-15 minutes of scrolling.
2. Export that spreadsheet as CSV, load it into Apify (an Instagram-post scraper/downloader) to bulk-download the actual video files.
3. Feed that CSV into Airtable, which triggers a Zap that sends each video into Bannerbear (a programmatic image/video templating tool).
4. Bannerbear auto-applies a template: video shrunk ~80% with a white border, and an auto-incrementing text overlay like "Day [N] of finding good [niche] content" — the day number increments automatically per video.
5. Finished videos are dumped into Dropbox, then bulk-uploaded from Dropbox into Hopper HQ, which auto-schedules them across the posting calendar.
- Total active time: roughly 45 minutes to produce a full month's worth of content.
- The "Day N of..." persistent-caption format is called out as a specific, proven high-converting hook: he tracks every video's stats and reports a 3-6x difference in view-to-follow conversion rate for videos using this "Day N" framing versus not, because viewers who like one video assume there's a series and go check the profile.
- For a separate channel (business-ideas shorts), the host similarly mass-produces "verbal call to action" (VCA) intro clips ("Day 67 of genius business ideas," etc.) in batches of ~50 in one sitting, reuses each clip multiple times, and once numbering gets too high just resets to "Day 1" and starts over — audience doesn't track/care about exact numbering; if someone does comment about it, that's just extra engagement.
- Strategic placement: VCA/hook clips are best placed right before a compelling moment in the video, or viewers swipe away.

## Copyright / platform risk notes (Sam's experience)
- YouTube enforces copyright strikes strictly: 3 strikes within 60 days results in full account deletion. He got 2 strikes within 2 weeks despite believing his content (overlaying his own face/commentary on others' clips) qualifies as fair use because it adds value/context — but automated copyright-claim systems don't evaluate fair use nuance, and disputing claims is difficult.
- Facebook also enforces copyright fairly strictly; Instagram and TikTok are described as much less strict about this.
- One dispute was resolved simply by contacting the original creator directly — but note the creator's specific objection was to monetization (thinking there was a paid course attached), not the repost itself, implying reposts draw less friction than reposts tied to paid products.
- Mitigation practiced on one of his other content accounts: explicitly crediting original creators via username (pulled during the Apify scrape) in an Airtable-generated caption formula, e.g. "shoutout [username] for the [X] recipe" — niche creators reportedly often send thank-you DMs for the exposure/publicity rather than complaining.
- Overall risk framed as "law of large numbers" — inevitable that some claims/strikes will happen at volume, but not something Sam actively worries about or plans around.

## Idea 3: Low-cost digital product "general store" (info products)
- Proposed model: a single branded Shopify storefront selling many small, cheap digital PDF guides/how-to products across different niches (e.g., sourdough baking, running form, personal finance spreadsheets), similar to Etsy's digital-download marketplace — instead of the "one product store" model popularized by tools like SoloDrop during the dropshipping era.
- Sourcing niche ideas: mine Etsy's marketplace data for what's already selling (what people are searching/buying) as validated demand signals (examples cited: "how to make a better pizza," "how to make sourdough," "personal finance spreadsheets").
- Rationale for paid vs. free lead magnets: paid digital products get consumed/valued more than free opt-in lead magnets, which people rarely open after downloading — paying creates a perceived value that drives actual usage.
- Economics: digital products have no COGS, inventory, or fulfillment — P&L is essentially "revenue minus marketing spend." Sub-$100 digital products also carry very low customer-service overhead (compared to something like a $5,000 course, which demands more support).
- Testing approach: build a product in minutes with AI-generated content, test multiple different titles/positioning/pricing (examples: $17-$99 range) for the same underlying product by running separate Facebook ad campaigns per variant on the same Shopify store, then scale the winner. A/B testing is described as trivial this way — no need to build a new website per product idea.
- Example numbers discussed: a hypothetical $20 weightlifting PDF at scale — willingness to spend $100,000-$150,000/month on Facebook ads to generate $150,000-$200,000+ in sales if targeting a market of a million+ people, plus upside from repeat purchases (e.g., "Sourdough: The Sequel") to a fraction of buyers.
- Growth/legitimacy tactic discussed (with explicit ethical caveat raised in the conversation): attaching a well-known influencer's name/likeness to a digital product without their prior consent (e.g., "based on [name]'s techniques"), sending them an unsolicited percentage of sales via Venmo proactively, and continuing until/unless they object — framed as improving ad conversion via borrowed social proof, especially effective with smaller/micro-influencers who are both more likely to respond and more likely to value the extra income; hosts flag this is legally/ethically gray and that individual risk tolerance varies — safer version is crediting an approach/technique ("based on X's technique") drawing from their own publicly available content (e.g., an existing YouTube video transcript fed into an AI prompt to generate a derivative product) rather than directly implying endorsement.

## Idea 4: AI research/business-plan-as-a-service micro business
- Concept: a $10-20 one-time-payment "instant business plan" service/site where a customer types in their business idea and receives an AI-generated, deeply researched business plan/report in return.
- Justification: paying customers don't want to learn to prompt AI research tools themselves or pay $200/month for a tool like Deep Research just for one use — cheaper and easier to pay someone who has already tuned the prompting process to run it for them once.
- Related monetization idea raised live: gate/paywall long-form AI-generated business plans partway through (e.g., a $5-10 unlock at 20% in) since most newsletter subscribers who opt in never read the full document anyway — the ones who do are highest-intent and likely to convert. Referenced newsletter volume: 500-2,500 new signups per day.
- Parallel real-world case cited: Sam's separate SEO content agency (unlimitedcontent.com) already does a version of this for onboarding — exports a new client's website as PDF, pulls existing SEO data (via tools like Search Atlas or SEMrush), feeds all of it to ChatGPT with a detailed prompt to auto-generate an 8-10 page SEO content strategy with keyword clusters.
- Broader sales framing/insight from that agency's calls: prospects rarely dispute that daily blog content works for SEO, and rarely think the deliverable itself is proprietary — the actual sales objection is "I could just do this myself." The winning pitch reframes the offer as reduction of headache/time rather than uniqueness of the output: "You could spend $400/month on the same tools plus 10-20 hours/month doing it yourself, or pay us $400/month and never think about it again."

## Tools named
- Manus (AI agent tool, invite-only at time of recording) — used to append population data to a CSV (podcast downloads-per-city) and calculate per-capita download concentration; reported doing this faster than ChatGPT for the same task, prompting them to cancel a $200/month ChatGPT Operator subscription in favor of cheaper/better alternatives.
- ChatGPT Deep Research and Perplexity ($20/month) — used as substitutes for the pricier ChatGPT Operator/Manus for structured research tasks.
- Zapier (with a Chrome extension for saving Instagram videos to a spreadsheet while scrolling; also referenced for auto-engagement, e.g. auto-liking a target account's last 20 tweets on X/Twitter).
- Apify (bulk downloading of scraped Instagram posts).
- Airtable (data table that triggers Zaps and generates caption text via formulas).
- Bannerbear (programmatic video/image template generation).
- Dropbox and Hopper HQ (bulk upload and social media scheduling).
- 72hoursites.com (Sam's own templated website-build service, $2,500/site).
- ClickFunnels / "Read .com Secrets" referenced generically as an example of learnable funnel-building skill.
- ChatGPT used directly for personal use case: built a free custom 12-week workout plan by inputting body stats/goals/equipment/time — compared favorably to paying a personal trainer ~$150/hour.
- Search Atlas / SEMrush (SEO data tools referenced in the agency example).

## Overarching philosophy / caveats
- Core thesis of the episode: stop chasing billion-dollar ideas — a modest 6-month effort can realistically produce ~$200K/year and business ownership; treat smaller, testable ideas as legitimate "swings" rather than holding out only for massive outcomes.
- Facebook Ads Manager is called out (half-jokingly) as one of the best inventions of the century for quickly testing market demand — cheap, fast iteration lets you "print cash" once you know how to run it, and lets you validate an idea's market size before over-investing.
- Willingness to spend more on ads than the immediate per-unit margin justifies IF the addressable market is large enough and repeat-purchase/upsell rates are decent (a few examples given: 5-10% of buyers purchasing a second product).
- Both speakers acknowledge holding many more ideas ("laundry list" of ~20 ideas each capable of "mid-six-figures, probably seven") than they can execute — capacity/time (sales calls, existing business operations) is the actual bottleneck, not ideas.
