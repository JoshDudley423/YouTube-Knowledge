# Stop Guessing. Here's How to Build What People Will Actually Pay For
Channel: The Koerner Office | https://www.youtube.com/watch?v=IHy07Ny72kA | Published: 2025-12-21

Demonstrates a validate-before-you-build method: poll real business owners on what they'd pay for, then use a no-code AI ("vibe coding") tool to build the winning feature as a sellable SaaS widget, same day.

## Business idea

- **Sell a simple appointment-booking widget (SaaS/website add-on) to small businesses** — validated directly from a poll of business owners, then built and reskinned per-industry (e.g., dog groomers, tree trimming companies) in minutes using AI. Model: build one core widget, then swap branding/colors/name per industry vertical to resell the same product repeatedly.
- Secondary insight-driven idea: bundle a "super widget" combining appointment booking + contact form intake + price calculator + a basic chatbot layer as one premium offer ("the last widget your business ever needs"), rather than selling any single feature alone.

## Validation process (before building)

1. Used **PickFu** (paid survey/poll tool, targetable by audience type) to ask 100 US small-business owners: "What one website feature does your business need the most right now that you would be willing to pay for?" with 5 fixed choices: appointment booking, contact forms, testimonials, chatbot, pricing calculator — plus free-text explanation for each vote.
2. Emphasized asking what they'd **pay for**, not just want ("sell painkillers, not vitamins").
3. Early partial results (27/100 responses): appointment booking led clearly (11 votes) over testimonials/chatbot (6 each), contact forms/pricing calculator (2 each).
4. Final results (100/100): appointment booking ended **tied for first with chatbot**; pricing calculator 3rd, testimonials 4th, contact forms 5th — a meaningfully different picture than the early sample suggested.
5. Fed all 100 written free-text responses + vote totals into ChatGPT and asked "what can I learn from this that will help me make money?" ChatGPT's read of the qualitative data (not just vote counts):
   - Appointment booking is the one feature that cuts across nearly every business type — genuine, urgent pain ("I need clients to see my calendar," "I do tarot readings and need scheduling handled for me").
   - Contact form demand is really about hating messy email / wanting reliable structured lead flow — but respondents expect forms to be free, so don't sell forms standalone; bundle them into the main product so they feel included.
   - Chatbot demand is inflated because people assume chatbots are hard/expensive to build; the segment that ranked chatbot #1 is likely willing to pay the most, precisely because of that perceived complexity — even though it's not actually their top real want.
   - Recommended top-of-funnel ad angle: lead with scheduling/booking (strongest genuine pain), then upsell the other modules once the customer is in.

## Build process (no-code / AI)

- Used **Mocha** ("vibe coding" tool — natural-language prompts build working software, no coding required), $20/month plan.
- Workflow: wrote an initial plain-language prompt describing the desired booking widget (embeddable, appointment/quote/call booking, easy to reskin colors/name per industry) → pasted that prompt into ChatGPT and asked it to rewrite/improve the prompt for a vibe-coding tool ("ask it what to ask it" — his stated favorite AI framework) → pasted the expanded prompt into Mocha.
- Mocha delivered a working multi-tenant booking widget system ("BookFlow") in **4 minutes 47 seconds** (10 files, 12 changes) from the first detailed prompt, published live at a mocha.app subdomain with custom-domain support.
- Fed ChatGPT's poll-analysis output back into Mocha as a new prompt ("expand this into a super-widget system based on the analysis below") — Mocha built out a full marketing site plus booking, contact-form, and price-calculator modules in response.
- Final refinement prompt: "just make one really good appointment booking widget for dog groomers, nothing else" — took ~18 minutes total and produced a working demo with service tiers (basic bath & brush $45/60min, full groom $75/90min, nail trim $15/15min) and a booking flow with date/time picker, name/email/phone/pet-name/breed/size/notes fields — all inferred by the AI without being explicitly specified.
- Re-skinning for a different vertical (e.g., tree trimming) took roughly one more prompt.
- Total time from poll to working prototype: about 16-20 minutes of active build time (excluding poll response wait time).

## Concrete numbers

- Mocha subscription: $20/month.
- PickFu poll cost scales with number of response options and targeting; not given as an exact dollar figure but described as "getting a little pricey" for 100 targeted small-business-owner respondents with 5 named options plus free text.
- Cost comparison for hiring a dev team to build an equivalent MVP normally: at least $10,000 (US-based) or at least $5,000 (overseas, e.g. Philippines/Ukraine), plus weeks of back-and-forth revisions — versus a few prompts and under 20 minutes with AI tooling.

## Caveats / risks

- Early partial poll data (27 of 100 responses) gave a different leading answer than the full 100-response result — small early samples can mislead; wait for a fuller sample before committing to build direction.
- Explicitly warns against the "if I build it, they will come" mentality — building without validating demand first is called out as the default failure mode for new business ideas.
- Face-value poll results (raw vote counts) can be misleading vs. what people will actually pay for — cites Henry Ford's "faster horse" quote as the classic warning against literally building what customers say they want.
- If you don't have an existing audience to poll, the video suggests using a paid targeted-poll tool (PickFu) or informally asking friends/family/Twitter/Reddit/Facebook groups instead — but a small informal sample is weaker signal than a larger structured one.
- No claim that this specific widget is already selling/generating revenue — this is presented as a build methodology and live demo, not a proven, monetized business yet.
