# The Easiest Way to Build a Local Newsletter
Channel: The Koerner Office | https://www.youtube.com/watch?v=Y0MRAjVp_Uo | Published: 2025-12-05

Live, real-time build of a local newsletter business for a randomly-generated city (Roswell, New Mexico, ~48K population) using Beehiiv and Meta (Facebook/Instagram) ads. Highly concrete step-by-step tutorial with real numbers from the live run.

## Core business model
- Customer acquisition = newsletter subscribers acquired via Meta (Facebook/Instagram) lead ads.
- Monetization = selling ad/sponsorship placements in the newsletter to local businesses (the newsletter's own subscriber base, since ~10% of people are business owners, is your sponsor prospect pool).
- Framed as one of the most approachable businesses to start because customer acquisition (normally the hardest part of any business) is unusually straightforward and paid-ads-driven here.

## Revenue math (CPM-based sponsorship model)
- Newsletter ad pricing is typically sold on a CPM basis (cost per thousand "opens"/eyeballs, not raw subscribers).
- Industry CPM range cited: $20-$100; example used throughout: $50 CPM.
- Worked example: 10,000 subscribers, 50% open rate = 5,000 opens per send. At $50 CPM: 5,000/1,000 × $50 = $250 revenue per single email send.
- Frequency scaling: sending 1x/week at 10,000 subs = $250/week (~$1,000/month). Sending 5x/week (M-F) = $1,250/week (~$5,000/month) — but publishing more frequently is proportionally more work and puts your sponsor-selling/management burden up 5x too.
- Explicit advice: start at low frequency (weekly) and increase later — starting "gung-ho" at high frequency and then dropping off is described as more damaging to morale than starting conservatively and scaling up on momentum.
- This CPM-ad revenue is described as just the baseline — additional revenue can come from affiliate links, merch, in-person events, or launching a separate business the newsletter promotes (not covered in depth in this video).

## Customer acquisition math (Meta ads for subscribers)
- Target CAC example used: aim for ~$2 per acquired email subscriber via Meta lead-gen ads.
- Value-per-subscriber calculation: at $50 CPM and 50% open rate, each subscriber is worth ~$0.50/month in ad revenue potential. Assuming an industry-standard ~24-month (2-year) average subscriber lifetime: $0.50 × 24 = $12 lifetime value per subscriber.
- If cost-per-acquisition is $1, that implies a 12x return on ad spend over the subscriber's lifetime (realized gradually over 2 years, not immediately — cash flow timing must be managed).
- Local newsletters are specifically called out as having unusually low acquisition costs because the targeting is so precise/relevant (can target only people in/near/moving to a specific town).
- Actual live results from this video's real ad campaign: an initial $100/day Meta lead-ad budget produced 8 real subscriber leads within about 10 minutes of the ad going live, and 11 leads within roughly 90 minutes of total campaign runtime — an effective cost of about $1.15 per subscriber early on (before optimization), with the expectation this would fall to $0.40-$0.50/subscriber as Facebook's algorithm optimizes further. Presenter states this is unusually fast approval/lead flow compared to his ~10+ years of running Facebook ad campaigns generally, and explicitly flags it as an atypically fast/positive result, not a guaranteed outcome for every campaign.
- Recommended ad budget pacing: start higher (e.g., $100/day) for the first day only to get the algorithm learning quickly, then drop to a lower sustained daily budget (e.g., ~$20/day) once you have initial traction — expect the very first leads to be more expensive ($5-$15 each) before cost per lead drops.

## Step-by-step build process demonstrated
1. **Pick a location:** any US city/town with roughly 50,000+ population is workable (used ChatGPT to randomly generate a target city as a stress test — landed on Roswell, NM, ~48K population, below-average local economic indicators, explicitly chosen to demonstrate the model works even in a non-ideal market).
2. **Choose a platform:** Beehiiv (referred to as "Beehive" in the transcript) — an all-in-one email newsletter platform the presenter has personally used for 2 years, offering integrated website hosting, monetization (ad network, paid subscriptions, boosts/referrals), automation, segmentation, and analytics. Presenter discloses a referral/affiliate relationship (30% off for 3 months for referred signups) but states this is a platform he'd recommend regardless.
3. **Name and set up:** pick a simple, geography-based name (e.g., "Roswell Dispatch"); choose publish frequency (started weekly); select acquisition channel (self-identified as "influencer" in Beehiiv's onboarding); pick a pre-built website template (chose a "news site" style template over a blog style) and publish the site immediately without customizing colors/branding — explicit advice to skip design/branding polish early ("advanced forms of procrastination") and prioritize getting subscribers first.
4. **Sequence matters — get subscribers before writing content:** deliberately built the ad/acquisition funnel before writing a single newsletter issue, on the reasoning that customer acquisition is the "scary" bottleneck to tackle first.
5. **Set up a Facebook Page for the newsletter** (a dedicated page specifically branded to the newsletter, not the host's personal/business page) — needed to run Meta lead ads under a consistent, relevant brand name so viewers recognize the newsletter name in the ad.
6. **Build the Meta ad campaign for lead generation:**
   - Campaign objective: "Leads," using Meta's Instant Forms (not sending to an external website) to minimize friction — form asks ONLY for email (no name/phone), maximizing conversion rate at the cost of lead "quality"/intent. Explicit tradeoff discussed: more form fields = higher-intent leads but lower conversion rate (~10% conversion with just email vs. dropping to 1-5% with added fields like phone/budget). Presenter explicitly optimizes for low friction/high volume ("momentum over focus") for this use case.
   - Targeting: geography set narrowly to the target town/city; left age/gender targeting to Meta's automatic optimization; enabled the "reach people likely to respond even outside the specified location" option (catches people using VPNs or those about to move to the area).
   - Placements: left open to all Meta placements (Facebook, Instagram, Audience Network) rather than restricting.
   - Did NOT enable "Advantage+"/dynamic AI-driven creative optimization, preferring manual control during the initial testing phase to save money before letting AI targeting take over later.
   - Ad creative: shot a simple ~16-second selfie-style video on iPhone using Instagram's free "Edits" app — no editing skills required; added burned-in captions (most people watch ads muted) and an on-screen hook overlay for the first couple seconds. Total ad-creation time: about 3 minutes.
   - Ad copy: used ChatGPT to generate 3 variations each of headline, primary text, and description for A/B testing, then accepted Facebook's own AI-suggested refinements to that copy rather than overriding them, reasoning that Meta's algorithm has better data on what converts for this ad type.
   - Research tactic for ad inspiration: used Meta's public Ad Library (filtering for "active" ads containing the word "newsletter," media type video, running continuously since before a cutoff date months earlier) on the logic that ads still running months later are very likely to be performing well (advertisers pull underperforming ads quickly).
   - Skipped a formal privacy policy requirement by linking to a generic/placeholder privacy policy page found online, on the reasoning that essentially no one actually reads it.
7. **Connect the Facebook lead form to Beehiiv via Zapier:** set up a Zap with trigger = "New Lead" (Facebook Lead Ads) and action = "Add Subscriber" (Beehiiv, using a Beehiiv API key generated from account settings). Also set a UTM source tag (e.g., "Facebook ad") on new subscribers added this way, enabling segmentation by acquisition channel inside Beehiiv.
8. **Write and send a welcome email** (used ChatGPT to draft copy) that fires automatically to new subscribers; also manually enrolled subscribers who joined before the automation was fully wired up, via a dynamic segment (auto-updating list based on the UTM Facebook-ad tag).
9. **Write the actual newsletter content:** used ChatGPT for local research prompts (e.g., "what are the coolest places and restaurants in Roswell?") as a starting point for content ideas; referenced an existing successful local newsletter for a different city (a friend's "Naptown Scoop" in Annapolis, MD, also on Beehiiv) as a structural model — noted it already had a paying sponsor and local business announcements.

## Additional growth/monetization features highlighted within Beehiiv (not all deployed in the demo, but described)
- **Recommendations:** mutual cross-promotion network between newsletters (recommend others, they recommend you back).
- **Boost:** a paid, pay-to-play referral network where one newsletter pays another (per new subscriber, e.g., $4/new subscriber cited as an example rate seen in the platform) to promote it to its own subscribers after they sign up — an alternative acquisition channel to running your own Meta ads.
- **Referral program:** built-in mechanic where subscribers earn a reward (e.g., a physical item, discount code) for referring others; explicit tip — set the reward threshold as low as "refer just 1 person" rather than a higher bar like 5, since a low bar is far more likely to be acted upon.
- **Magic links:** one-click subscribe links (no manual email entry required).
- **Beehiiv Ad Network:** advertisers seeking presence in specific local markets can be matched with your newsletter automatically by the platform.
- **Paid subscriptions:** ability to charge subscribers directly (e.g., $3-$10/month) for premium/exclusive content, either instead of or alongside the free tier.
- **Print-on-demand products:** sell branded merchandise (e.g., newsletter-branded T-shirts) fulfilled through third-party print-on-demand integrations.
- **Post-subscribe surveys:** built-in survey feature to ask new subscribers demographic/interest questions (e.g., how long they've lived locally, industry they work in) immediately upon signup — presenter cites >50% completion rates when the survey is sent at the moment of subscribing, versus near-zero response if delayed even an hour, because new subscribers are most engaged right at signup.
- **Polls:** used to solicit reader content preferences directly (e.g., "what do you want to know about?" with options like best local deals, local news, opinion pieces, stories from residents) — presenter says this feature is underused by most newsletter operators despite being freely available.

## Sponsor/monetization tactic once you have subscribers
- Simple practical approach described: email your full subscriber list directly asking who wants to advertise, then sell ad placements individually over the phone starting around $50-$200 per placement and adjusting up until you start hitting resistance/rejections — described as a straightforward, low-tech way to begin monetizing once there's an audience.

## Caveats
- The extremely fast/cheap lead results shown in this specific live demo ($1.15/lead within 10 minutes, 11 subscribers within ~90 minutes) are explicitly flagged by the presenter as unusually good even by his own extensive (10+ years) paid-ads experience — not something to assume as a guaranteed baseline outcome for every market/niche.
- The demo deliberately chose a below-average-income, small, unfamiliar city (Roswell, NM) specifically to show the model isn't dependent on picking an especially favorable or wealthy market.
- Revenue/ROI math presented (12x LTV:CAC, $5,000/month at 10,000 subscribers) depends on successfully closing real sponsor deals at the assumed CPM rates — the video only demonstrates the subscriber-acquisition side, not proof of actual completed sponsor sales for this specific newsletter.
