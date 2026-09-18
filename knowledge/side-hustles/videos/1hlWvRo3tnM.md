# How To Make Money by Building Simple Apps for Businesses (AI + No-Code)
Channel: The Koerner Office | https://www.youtube.com/watch?v=1hlWvRo3tnM | Published: 2025-11-16

Chris Koerner and Brandon Doyle race to each vibe-code a working lawn care quote generator web app in under an hour using Emergent (an AI "vibe coding" tool), to show how cheap/fast it now is to build custom business software.

## The idea
- Build small, single-purpose web apps (quote calculators, lead-gen forms, booking tools) for local service businesses (lawn care, roofing, painting, fencing, HVAC, etc.) using AI "vibe coding" tools instead of hiring developers or paying for bloated SaaS (Salesforce, Jobber, HubSpot).
- Pitch: most small businesses use only 2-3 features of a $400-500/month platform like Jobber; a custom-built single-feature app can be built for ~$10-20 and either given away as a lead magnet/foot-in-the-door, sold as a cheap SaaS product, or used as an internal tool.
- Explicitly generalizable: same "instant quote calculator + lead capture" logic applies across roofing, painting, fencing, dog grooming, HVAC, and other home-service verticals.

## Numbers
- Emergent monthly plan: $20/month (one got $10 off as a new-user discount, i.e. paid $10 for the first month).
- Total cost to build and deploy each demo app: ~$10 in usage credits (each started with 110 credits; one ended around 42 credits left, the other 51 left).
- Chris's app took 9 prompts / 354 words total; Brandon's took 6 prompts / 211 words total — both built a live, deployed working app in under an hour.
- Comparable existing SaaS pricing cited as the alternative: Jobber's SMS-capable tier runs ~$400-500/month.
- Example quote logic built: base rate ~$50 per 1,000 sq ft of lawn (or 1.6 cents per net square foot after subtracting house footprint and ~10% for driveway), with add-ons like +25% for hedging, discounts for recurring service (5% off biweekly, additional 5% off weekly, or tiered discounts like 20% off for 2x/month, 40% off for 3+/month), a $40 minimum job price override, and an optional weed-eating add-on priced at 20% of the total.

## Process / steps described
1. Pick a vibe-coding tool (Emergent used here; others named: Bolt, Replit, Cursor, ChatGPT/Codex-style tools).
2. Give a simple, casual, imperfect natural-language prompt describing the business and what the app should do — the presenters explicitly argue elaborate "expert prompt engineering" (e.g. viral "you are trapped in an Iranian prison" style prompts) is unnecessary; a short, typo-ridden prompt works fine.
3. The tool asks clarifying questions (what services, how pricing should be calculated, how quotes/leads should be handled, design preferences) — answer them like you're briefing an entry-level employee.
4. Iterate conversationally to refine pricing logic (walk it through your own manual math/example calculation to save correction prompts), request scaling/tiered discount logic, and set minimum price floors to avoid $0 quotes.
5. Choose a lightweight backend option to avoid friction — e.g. skip integrating a real database/CRM (Supabase, Google Sheets OAuth, SendGrid + 2FA) and just have it email you the lead, or pipe form submissions to a plain Google Sheet.
6. Ask the chat interface itself (not Google) how to deploy — the tool can walk you through publishing/launching a live test URL directly in the same conversation.
7. Add a Stripe payment link placeholder if you want to eventually take payment on the spot.
8. Once live, treat form submissions as leads: call/text the customer immediately, potentially undercutting the app's own quote slightly to close the deal fast (speed-to-lead as a competitive edge, since most home-service owners are busy in the field and slow to respond).
9. Consider layering a missed-call/missed-text auto-responder (e.g. via Zapier, Lindy, GoHighLevel, or possibly within Emergent itself) that auto-texts the quote-form link if a call/text goes unanswered for 5 minutes.

## Tools/platforms named
- Emergent (the vibe-coding tool used; described as ranking #1 in coding-agent benchmarks, has its own built-in database so you don't need to separately integrate Supabase).
- Other vibe-coding tools mentioned as alternatives: Bolt, Replit, Cursor.
- ChatGPT (used to brainstorm app ideas and research the vibe-coding tool before starting).
- Google Sheets (used as lightweight lead storage/backend instead of a full database).
- Zapier / Lindy / GoHighLevel (suggested for building a missed-call/text-back automation layered on top of the app).
- Stripe (payment link, only stubbed in, not actually connected in the demo).
- Airwallex (sponsor; multi-currency business account, not related to the core method).

## Design/UX trade-offs discussed
- Friction vs. lead quality trade-off: showing an instant price before collecting contact info (like one demo app) risks losing leads who don't like the price and simply leave without giving contact info; collecting contact info before showing price captures more leads but adds friction. No universally "right" answer — depends on the industry and how much you're relying on price transparency vs. lead volume.
- Adding extra qualifying questions (e.g., "is the house one or two stories?") can improve quote accuracy but reduces completion rate (estimated ~5% fewer responses per extra question) — treat every added form field as a cost.
- Visual design/polish (one demo styled after Airbnb's aesthetic) was judged to matter less than backend logic correctness for the underlying business value, though it can meaningfully help a landing page's conversion/trust when the app itself is customer-facing (vs. an internal tool sent directly to leads).

## Caveats / risks
- Quote math needs careful sanity-checking: one demo app initially produced obviously wrong prices (e.g., $0 for a 5,000 sq ft lot, or $2,450 to mow one acre) that required manual correction prompts — AI-generated pricing logic is not automatically correct and must be validated against real-world reference points.
- The presenters are explicit non-coders ("vibe coding aficionados," no formal coding background) — this is presented as evidence anyone can do it, but also means the outputs shown are demo-quality/unpolished (e.g., dead Stripe link, no multi-story house handling, discount scaling issues) rather than production-hardened.
- Keeping the app running long-term on Emergent costs $20/month (or you can export/self-host elsewhere); the $10 build cost is a one-time/incremental usage cost, not the full ongoing cost of running it.
- This is explicitly framed as a proof-of-concept/speed demonstration, not a fully validated, revenue-generating business — no real customers, traffic, or monetization was tested within the video itself.
