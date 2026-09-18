# Watch This Overlooked AI Tool Build Me a Business in 23 Minutes
Channel: The Koerner Office | https://www.youtube.com/watch?v=uQKVxCQJPoA | Published: 2025-09-12

## Core idea: build and launch a niche invoicing SaaS ("InvoiceFreely") using AI "vibe coding" (Replit Agent 3) with no manual coding
- Business concept: free/freemium invoicing software targeted at a specific underserved niche (contractors/home service businesses — tree trimmers, roofers, general/subcontractors), letting customers create and send invoices via natural language (SMS or chatbot) instead of a traditional CRM form (e.g., "send an invoice for $800 to Mary Smith for tree trimming services").
- Monetization: instead of charging a subscription, mark up the credit card processing fee. Stripe charges ~2.9% to process a payment; charge the end customer a slightly higher rate (e.g., ~3.5%) and keep the spread (~0.5% of every dollar processed) — customers can also mark it up further to their own clients. Framed as a "free" tool because there's no subscription fee, only a processing markup.
- Market size argument: invoicing software is a "crowded" market, but there are 33 million small businesses in the US that need it, most unhappy with existing solutions — even 100 customers at a modest volume could generate ~$1,000/month with near-zero marginal cost.

## Idea-validation method demonstrated (reusable for any niche)
1. Check Google Trends for the base search term (e.g., "invoicing software") over 5 years to confirm rising demand.
2. Go to google.com and type the term followed by a space to see autocomplete suggestions — autocomplete reflects a blend of search volume and trend direction. Read down the list for under-served/qualifying segments (in this case: "small business," "free," "freelancers," "contractors," etc.) to find a specific niche with signal.
3. Combine the trending qualifier ("free") with a specific under-served vertical ("contractors") to define a defensible micro-niche rather than competing generally.

## Build process (Replit Agent 3 / "vibe coding")
- Replit Agent 3: an autonomous AI coding agent that can work up to ~200 minutes per task in the background, self-test built apps in a real browser, auto-fix issues, and even spin up sub-agents — claimed 10x more autonomy than Agent 2.
- Single "one-shot" prompt used: "Build me a free invoicing software for contractors where my customers can use natural language either via email or a chatbot window to create and send invoices that have a Stripe link attached." Build took Replit ~20+ minutes.
- Required only two/four API keys total: Stripe (secret key "sk..." and publishable key "pk...", from Stripe's Developer > API Keys page) and Resend (transactional email infrastructure, not a marketing tool like Beehiiv/Mailchimp — used for actually delivering the invoice emails), plus OpenAI (for natural-language parsing) — all entered into Replit's "Secrets" panel.
- After building: bought a domain via Namecheap (invoicefree.com, ~$11) chosen to be short (4 syllables), easy to spell, and not overly niche-locked (works for any business that invoices, not just contractors).
- Domain/email setup steps needed to actually deliver emails: verify a sending domain in Resend (add custom MX, TXT/SPF, DKIM, and DMARC DNS records via the domain registrar) so the email service trusts you're not a spammer; connect the purchased business domain to the Replit app via DNS "A" and "TXT" records; a no-reply email address on a domain you don't use for regular email is fine for transactional sending.
- Common early hiccups fixed by prompting Replit conversationally: emails not sending because no "from" address environment variable was set; from-domain not matching the domain actually verified in Resend; first test invoices landing in spam (attributed to using a fresh/unestablished "no-reply@...ai" sending domain — noted as fixable but not resolved in the video).
- App came out with a working dashboard (invoices sent, amount paid/pending, customer count) that wasn't explicitly requested — added automatically by the agent.

## Business/launch philosophy emphasized
- MVP framing: "build a scooter, not a half-finished car" — launch a rough, even embarrassing v1 immediately rather than waiting for perfect design, full compliance (e.g., SMS/A2P 10DLC registration, which takes ~2 weeks and requires a Twilio number at ~$1/month), a logo, or a marketing plan. Excess pre-launch polish is framed as the main reason people never actually launch.
- Full SMS invoicing (texting "send Mary Smith an invoice for $800" from a job site) was designed into the concept but deferred for a v2 because of the multi-week A2P registration process; email/chatbot was used as the immediate MVP channel instead.

## Tools/platforms named
- Replit (Agent 3) — AI app builder / "vibe coding" tool.
- Stripe — payment processing and checkout links.
- Resend — transactional email API/infrastructure.
- OpenAI API — natural language processing of invoice requests.
- Twilio — SMS/phone number provisioning (for a future SMS feature, not built in this video).
- Namecheap — domain registration and DNS management.

## Caveats/risks
- Explicitly stated this was a demo, not a serious business the host intends to run ("Am I planning on making this a business? No"), though he acknowledges that publishing a working vibe-coded app to an audience effectively makes it public/usable regardless of intent.
- Invoicing software is a genuinely crowded/competitive market — the differentiation here is narrow niche targeting (contractors) plus a natural-language interface, not a fundamentally new product category.
- New sending domains can land in spam by default; deliverability requires proper DNS authentication (SPF/DKIM/DMARC) and warming up the domain — not something solved instantly.
