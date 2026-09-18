# Missed Every Gold Rush? This One Started Just 2 Days Ago
Channel: The Koerner Office | https://www.youtube.com/watch?v=-Whqjr6ZnmY | Published: 2025-04-25

Interview with Jacob Posel (AI hacker/builder) recorded the same day OpenAI released its image-generation API (April 23, 2025).

## Business ideas discussed

- **Consumer "theme" image apps**: narrow, single-purpose wrappers around image generation (e.g. "Giblify me," "anime me," "what would I look like older / with a beard / 30 lbs lighter," "what would our baby look like" by combining two people's photos). Pitch: feeds narcissism/vanity, has high viral potential (e.g. via TikTok). Monetization: subscription + credit packs for extra generations.
- **AI-generated ads for performance marketers/ad agencies** — framed as the biggest near-term opportunity:
  - Ad algorithms increasingly reward volume/variety of creative over a single highly-optimized ad, so marketers need cheap ways to mass-produce ad variants.
  - Process: take a "winning" ad that has fatigued, feed it to the AI as a reference image along with the client's product image, and generate new variants to refresh the ad account.
  - Go-to-market: don't sell to individual brands — partner with/piggyback on existing ad agencies who already have a portfolio of paying clients ("labor/software arbitrage" business model). Cold-DM agencies offering to whitelabel the service.
  - Aggressive tactic suggested: scrape lists of ad agencies (especially ones not yet using AI), find a client in their public portfolio, proactively generate several unsolicited ad variants for that client, and send them to the agency with a link to a tool/wrapper you built (e.g., in Replit) so they can self-serve going forward.
- Real estate / interior design: take a photo of an empty room and use AI to "furnish" it virtually; also mentioned improving lighting/vibe of Airbnb listing photos (raised as ethically dicey — got negative reaction on Twitter).
- Tree service example (Chris's own prior business): generate personalized images of a homeowner's house with "a tree fallen on it" to scare-market tree-trimming services.
- Stock imagery: generate any conceivable stock photo on demand — doesn't need a reference image.
- Micro-personalization in marketing: because creative generation is no longer the bottleneck, brands can target far narrower personas (e.g. "upper-middle-class women from a specific ZIP code in southern Florida" instead of "middle-aged women in America") with unique ad creative for each, at scale.

## Monetization / pricing approaches for the ad-agency angle

- Charge per-ad (a per-ad "value" the agency already has baked into how it bills its own clients), then agency marks it up when reselling.
- Or a flat monthly retainer regardless of ad volume, to reduce the agency's downside and make budgeting predictable for brands (brands don't want a surprise bill).
- Or a hybrid: base retainer + per-accepted-ad fee.
- If building a self-serve tool for agencies instead of doing the work manually: suggested price point ~$200/month for "unlimited" image generation (need to watch/cap API credit costs).
- Posel's own recommendation: in the early days, don't build a self-serve tool — do the service manually for the agency (e.g., deliver a Google Drive folder of finished ads in whatever format the client already expects). Removing friction/effort from the agency's side matters more than software polish.

## Prompting / process tips for AI image generation

- A single reference image is worth far more than a text description — provide reference images whenever possible (can supply multiple).
- For reproducing/adapting an existing visual (e.g., cloning a competitor's ad style for your own product): supply (1) the reference ad as one image and (2) your product photo as a second image, and explicitly tell the model which is which.
- Reference images matter most when reproducing something specific; the text prompt matters most when creating something novel/fantastical, especially using widely-known characters/themes (the model already "knows" e.g. Harry Potter, so no reference image is needed there).
- At time of recording, Sora was considered better than ChatGPT's own image generator for practical use: Sora generates 4 image variants in parallel (more choice per attempt) and was believed (unconfirmed, personal intuition) to use a higher-quality underlying model than the one downgraded into ChatGPT for cost/compute reasons. Sora's public "explore" library also lets you see other users' exact prompts — useful for idea generation and reverse-engineering what's working.
- Rough estimated "good output" hit rates at the time: text ~90%, images ~60%, video ~5% — video was judged still a "gimmick," not yet worth building a business around.

## Caveats / risks

- This is a fast-moving/first-mover opportunity — the guest emphasizes urgency ("give it a week" and features existed that didn't a week prior); much of the specific advice may go stale quickly as OpenAI/others ship better tools.
- Ad-generation-as-a-service is not guaranteed easy money: "prompt engineering" best practices for producing a high proportion of viable ads were described as still unsolved at time of recording — without solving that, expect a spike in interest followed by high churn rather than a durable business.
- The "improve my Airbnb/room photos" angle drew ethical pushback (misrepresenting a space) when floated publicly.
- Guest deliberately stays narrowly focused on one product/company rather than chasing every idea raised in the conversation, citing spreading himself thin as a personal failure mode.
