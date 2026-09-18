# Forget Dropshipping. This Side Hustle Is 10x Easier (and Smarter)
Channel: The Koerner Office | https://www.youtube.com/watch?v=6QThtf2K1r4 | Published: 2025-08-17

Guest: Cody Schneider — has run Etsy print-on-demand (POD) stores for 10+ years.

## The idea(s)
1. **AI-automated Etsy print-on-demand**: research bestselling Etsy products, use AI (ChatGPT image generation) to create new but similar graphics, and mass-list them on Etsy under the same proven keywords/pricing — essentially a "parasite SEO" play riding Etsy's existing domain authority/traffic instead of building your own audience/traffic.
2. **Merch-store agency for content creators**: cold-pitch creators (especially long-form podcasters/YouTubers) a done-for-you merch store built around their own catchphrases, charging a percentage of revenue with zero upfront cost to the creator.

## Why print-on-demand over dropshipping (explicit comparison)
- Dropshipping downsides cited: goods ship from China (~1 month delivery), often poor quality/packaging mismatched to branding, high chargeback/dispute risk, thin margins because it requires heavy paid ad spend, product trends (e.g., a viral TikTok gadget) can die before the order even arrives, and a dropshipping business reportedly cannot be resold.
- Print-on-demand upsides: no inventory, no sunk cost if a design goes stale, printers (e.g., Printify) have multiple US factories so shipping is a few days not a month, and — importantly — a POD business (IP/designs + cash flow) can actually be sold on a marketplace, unlike a dropshipping operation.

## Process — Etsy AI print-on-demand
1. **Product research**: use Everbee (a "Jungle Scout for Etsy" Chrome extension/web app) to search a generic keyword (e.g., "horse sweatshirt") on Etsy and identify bestsellers by estimated search volume, sales, and revenue. Export the winning listings as a CSV.
2. **Generate new graphics**: screenshot a winning product's image, feed it to ChatGPT's image generation (via the app or API) to create a new, similarly-styled but original graphic. Vector/black-and-white style images work best because there are "fewer variables for ChatGPT to screw up." At scale, this is run through automation for e.g. 1,000 winning products → 1,000 new generated images at once.
3. **Reuse the winning listing's metadata**: same keywords in the title, same description structure, same pricing as the proven bestseller — you're not guessing, you're copying a data-validated formula.
4. **Avoid trademarked words/phrases** in designs and titles (e.g., "Horses Make Me Happy" is trademarked by someone who actively polices Etsy sellers) — get 3 trademark strikes and Etsy can shut down your shop and flag your linked social accounts. Stick to generic, non-phrase-based concepts since this is a volume game (target: 10,000+ live listings).
5. **Post-process images**: remove the background (mentions tools like "remove.bg"-style services) before sending to the print fulfillment platform.
6. **Fulfillment**: upload the final image to Printify (or Printful), bulk-select variations (shirts, sweatshirts, mugs, etc.), and one-click publish to Etsy. No inventory is ever held — orders route via API to a fulfillment warehouse (example given: Kansas City) that prints and ships directly to the customer.
7. **Automate the whole pipeline with n8n**: connect Google Drive (source images) → ChatGPT API (generate new image) → save to a new Drive folder → (optionally) Printify → Etsy. Because writing n8n's underlying JSON workflow config by hand is hard for non-coders, the trick is to have an AI (Cody used Perplexity Pro, which runs on Claude) write the n8n JSON flow from a plain-English description, then paste it directly into n8n — works essentially out of the box. Scraping tools like ScrapingBee or Apify can pull the source images/data.
8. Manual involvement can be reduced to just the "research" step (finding winning products) — Cody says he doesn't fully automate that part because he finds it fun, not because it's technically impossible; everything else (generation → background removal → fulfillment → listing) can run unattended.

## Numbers / economics
- Listing cost on Etsy: roughly $0.20-$0.30 per listing.
- Ongoing costs are just: the Everbee subscription, per-image AI generation cost, and Etsy's small listing fee — described as "basically zero startup cost."
- Expect an 80/20 outcome: roughly 20% of listings will generate the bulk of revenue; most individual listings won't make meaningful money, so volume (many listings) is the strategy, not picking one perfect design.
- Cody's realistic framing: an individual doing this seriously could plausibly make "another $100,000 a year" as a side strategy — not necessarily "crazy wealthy," but a legitimate, learnable, low-capital first business.
- Optional advanced move for a proven organic Etsy winner: replicate the design on a separate Shopify store and run Google Shopping ads directly against it, since you already know demand exists. Treated as an arbitrage/ROAS game — example economics given: $1 spent generates ~$2 in revenue, but if margin on that revenue is only ~20%, you're netting ~$0.20 per $1 spent; at high spend volumes ($100K+), that thin margin becomes meaningful in absolute dollars. Side tip: use a rewards credit card that gives elevated points on ad spend (cites Amex ~4x points, Chase Sapphire ~3x points on paid ads) to stack travel rewards from what is otherwise a low-margin ad-arbitrage tactic.

## Idea #2 — Merch-store agency for creators (worked example, "freestyle" brainstorm on the podcast)
1. Identify creators (especially long-form podcasters/YouTubers, not short-form, because long-form audiences have a deeper "parasocial" relationship built over repeated 60+ minute sessions) with meaningful followings.
2. Mine their content for recurring catchphrases (pull transcripts of their published videos/episodes and identify frequently repeated phrases) — these become the actual merch designs (e.g., a hat that says a phrase the creator repeats often).
3. Build the merch mockup/store BEFORE pitching — show the creator a tangible example (e.g., an actual mockup hat) rather than describing the idea abstractly, since people can't easily visualize an unbuilt concept.
4. Set up the actual store using a creator-merch platform like Fourthwall (mentioned as what Chris himself uses for tkostore.com) — the whole build (domain + store) can reportedly be done in about a weekend for roughly the cost of a $10 domain plus Shopify's free trial.
5. Pitch: build and manage the entire store (designs, logistics) for free, in exchange for a percentage of revenue (example figure floated: 5% of revenue), no upfront cost to the creator.
6. Outreach at scale: scrape creator emails (e.g., from YouTube channel "About" pages) using a tool like Apollo (referred to informally as "amplify"/mis-said in the transcript) or Apify's YouTube channel email scraper; validate emails with an email-verification tool (Million Verifier) before sending, since sending to invalid addresses damages your sending domain's deliverability reputation; run the actual campaign through a cold email tool like Instantly.ai. Subject line suggested: "i built you a merch store" (all lowercase). Ask for a simple yes/no reply, then send the store link only once they respond yes.
7. Expected outcome distribution, per Cody's real experience with these kinds of automated outreach plays: either you fall well short of your revenue projections but still learn a lot, or you wildly exceed them (10x) because demand turns out to be much bigger than expected — outcomes are described as rarely landing near the projected middle.

## Tools/platforms named
- Everbee — Etsy product research tool (search volume/sales/revenue estimates), Chrome extension + web app.
- ChatGPT (app and API) — AI image generation for new print-on-demand graphics; also used to generate FAQ/content ideas in general.
- Perplexity Pro (runs on Claude) — used to generate n8n JSON workflow configs from plain-English prompts.
- n8n — automation platform (compared to Zapier/Make.com but more powerful/AI-native; harder to use directly due to JSON-based flow configuration, hence the AI-generation trick).
- Make.com — alternative no-code automation tool, easier UI than n8n but less flexible.
- Printify / Printful — print-on-demand fulfillment and multi-SKU (shirts, hoodies, mugs, stickers, notebooks, jackets, baby items, custom embroidery) publishing to Etsy.
- Google Trends and Google's "site:etsy.com/market" search trick (with Tools > Past 24 hours filter) — used to spot newly trending product categories on Etsy before they saturate, catching trends as they migrate from viral short-form video (e.g., TikTok) into purchase intent.
- ScrapingBee / Apify — general web scraping tools for pulling source images/data.
- Fourthwall — creator-merch platform (used for the tkostore.com example).
- Apollo (garbled as "amplify" in transcript) / Apify's email scraper — for finding creator contact emails.
- Million Verifier — email address validation before cold outreach.
- Instantly.ai — cold email campaign sending tool.
- Amex / Chase Sapphire cards — for stacking points on ad spend.

## Caveats / risks
- Trademark abuse claims can get an Etsy shop banned and linked social accounts flagged — avoid word/phrase-based designs that might already be trademarked; stick to generic imagery.
- This is explicitly a volume/numbers game: most individual listings won't be profitable; success depends on producing enough listings that the ~20% that do work cover the rest.
- Not guaranteed to work: Chris explicitly frames this (and any "first business") as statistically likely to underperform expectations — the value is in gaining transferable skills (research, automation, e-commerce mechanics) even if this specific venture doesn't take off.
- Etsy Shopping-ad arbitrage plays can have thin per-dollar margins; only becomes meaningful profit at high ad spend volume.
- The creator-merch cold outreach idea depends on scale (sending to many creators) since individual response/conversion rates and per-shirt margins are both low — a single deal converting well can look like "only" ~$2,000/month, requiring several successful deals to add up to significant income.
