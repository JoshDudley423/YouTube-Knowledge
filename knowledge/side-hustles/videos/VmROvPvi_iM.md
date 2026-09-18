# Start an Online Business With Just a Prompt
Channel: The Koerner Office | https://www.youtube.com/watch?v=VmROvPvi_iM | Published: 2025-07-05

Interview with Eric Simons, co-founder/CEO of Bolt.new (a "vibe coding" AI app/website builder by StackBlitz), on using no-code AI tools to launch real software businesses.

## The core opportunity: non-technical people building and selling real software
- Bolt.new lets anyone describe an app/website in natural language and get a working, deployable full-stack application — including payments, backend/APIs, and mobile app builds (via an Expo/React Native integration) — typically within 5-10 minutes for a first working version.
- Framed as removing the single biggest historical barrier to entrepreneurship for non-technical, marketing/business-minded people: not knowing how to code.
- Bolt went from $0 to $20M ARR in its first two months after launch (per Eric) — company had been building a developer-focused product for 7 years and had only reached $700K ARR before pivoting; most new users were not developers (designers, PMs, salespeople, entrepreneurs), revealing a much larger addressable market than expected.
- Key differentiator claimed vs. other AI coding tools (Cursor, Replit, Lovable, Windsurf, v0/Vercel, Claude-based tools): Bolt focuses on end-to-end business-readiness (payments, real APIs, production design) rather than just producing a flashy MVP that still needs significant manual work to become sellable; and it runs a full dev environment directly in-browser (via 7 years of prior "StackBlitz" in-browser IDE technology) rather than booting a cloud VM per session, making it faster and more reliable to start/resume.

## Concrete monetization patterns observed among Bolt users
- **Agency/consulting arbitrage**: build client deliverables cheaply with AI, charge normal market rates. Cited example: a user ("CJ" on X) built a client dashboard for ~$9 in Bolt usage costs and charged the client $9,000 for it. Dashboards specifically are called out as something people will pay heavily for because "everyone loves a dashboard that makes them feel in control" — Chris's explicit suggestion: "go start a dashboard agency," aggregating a client's data via APIs into a custom interface.
- **Building your own SaaS/tool instead of paying for one**: e.g., a friend paid ~$30/month for a mediocre iPad teleprompter app; had Bolt build a better one in one prompt for roughly a couple of cents in usage cost, then owned it outright. General framework given: "whatever software tool you use that's overpriced or takes too big a cut, try to replicate it in Bolt — worst case you save money for yourself, best case you resell it to others."
- **Owning your own monetization infrastructure instead of using a hosted platform that takes a cut**: e.g., building your own course-selling website instead of using a third-party course platform that takes a 10-30%+ revenue cut; Eric noted his own prior company spent 1-2 months just building payment/login infrastructure before AI tools existed to do it in minutes.
- **Building niche AI-powered SaaS products**: example cited — an early user named Paul built a full AI-integrated CRM using Bolt and is selling subscriptions to it.
- **Internal tools for existing businesses**: Fortune 500s and scaleups also use Bolt to build bespoke internal tools/prototypes faster (not just startups).
- **Non-commercial but high-value applications**: a hackathon participant in South Africa built an app to help people locate emergency medical services in an area lacking infrastructure for it, after a friend was injured and didn't know where to go.

## How Bolt is actually used / prompting practices (demonstrated live)
- Recommended workflow: start with a simple first prompt describing the app/business idea broadly to get layout, look, and feel right (e.g., "create a course-selling website for my course on X") before layering in functionality — mirrors normal product development (nail UX/flow first, then features).
- For complex functionality changes, break requests into smaller discrete prompts rather than combining many changes in one message, so you can roll back a single failed change without losing other progress.
- For visual/design direction: it's effective to give loose, "vibe"-based direction (e.g., "make it beautiful," "make this pop") rather than over-specifying every detail, since the underlying models are trained on best-practice examples and often produce good results from vague instructions.
- Screenshots work as direct design references — e.g., live demo showed dragging in a screenshot of Airbnb's homepage and prompting "make the theme look more like this," producing a close visual match in under 90 seconds despite Airbnb being in an unrelated industry.
- "Discussion mode" (a chat-only mode where the AI doesn't write code, just talks through a plan or debugs) is recommended for troubleshooting stuck/broken features — it asks clarifying questions and can instrument the code with logging rather than guessing at a fix; discussion-mode messages cost roughly 1/10th the price of a normal build message, making it cheap to use liberally for debugging without burning through a usage allotment.
- Design quality out of the box is generally strong enough that most builders shouldn't spend significant extra time hand-crafting UI/UX — Eric noted Bolt itself launched and scaled to $20M ARR without even having mobile-responsive design for months; the advice is to launch once value is real rather than polishing indefinitely, unless design itself is your core competitive differentiator.

## What's still hard / limitations noted
- Complexity scales with how far you move beyond basic CRUD (create/read/update/destroy) apps into real-time, algorithmically complex systems — the example given was an "Uber clone" requiring real dispatch/matchmaking algorithms and live GPS coordination, which is genuinely difficult regardless of tool, though the same idea scoped down (e.g., a simple 20-driver dispatch tool) becomes tractable.
- Achieving more complex builds benefits from at least conceptual understanding of what tools/services solve which problems and how to combine them, even without knowing how to code directly — Eric frames this as still valuable domain knowledge worth having as you scale up project complexity.

## Broader framing on skills/career strategy
- Eric's take on "should I learn to code": if you're early-career and torn between business/marketing and coding, lean into business and use AI vibe-coding tools as your coding shortcut; if you're already a skilled developer, lean further into coding + AI tools together rather than away from coding, since technical understanding gives more leverage in directing AI agents.
- Argued non-technical "vibe coders" can sometimes outperform experienced developers on early-stage exploration because they lack the developer's preconceived biases/assumptions about what's hard or not worth trying — used a marshmallow/toothpick tower-building experiment (kids beat executives by iterating quickly instead of over-planning) as an analogy.
- Bolt deliberately hires PMs/designers with strong end-user empathy because, as a team of experienced engineers, their own instincts increasingly diverge from what non-technical end users actually need — a caution that domain expertise (coding, in this case) can become a liability for understanding a mass non-expert market.

## Launch/marketing tactic discussed (of general use for any founder)
- Bolt's viral launch was driven entirely by one carefully-crafted tweet with zero paid marketing — built on top of an existing ~20,000-follower audience cultivated over years.
- Recommends the "Amazon PR/FAQ" approach: write the press release and FAQ for a product before building it, forcing clarity on why it's unique and what problem it solves, then work backward.
- The tweet itself: led with a comparison question framing why the product is different/needed (rather than technical detail up front), kept to a single tight tweet (not a long thread) with the core hook, and put the product URL directly in the first tweet (not hidden in a reply) — contrary to common advice to avoid links in a lead tweet (which can reduce algorithmic reach), Eric/Chris argue that if the tweet is compelling enough to keep people engaged on the platform (watching replies/videos) even after some click the link, it can still get amplified.
- Chose the ".new" domain (bolt.new) specifically because a ".new" domain must load directly into a working action/tool (analogous to docs.new opening a fresh Google Doc) — reinforcing an intentional zero-friction, no-marketing-page-to-scroll-past product experience.

## Numbers cited
- Bolt: $0 to $20M ARR in first 2 months post-launch (previous product: $700K ARR after 7 years).
- Bolt hackathon (June, referenced in the episode): $1M+ in prizes, 100,000+ participants, described as exceeding the prior Guinness World Record hackathon size (~66,000 participants).
- Example client project: $9 Bolt cost to build vs. $9,000 charged to the client (dashboard/reporting tool, delivered agency-style).

## Caveats / risks raised
- Chris discloses he uses a Bolt affiliate link (financial interest in referrals), same disclosure pattern as other TKO tool-focused episodes.
- Eric was explicitly skeptical of AI hype from 2022 through early 2024 (compared it to the web3/blockchain hype cycle he'd also sat out) and only built Bolt after Anthropic's Claude 3.5 Sonnet model showed a genuine step-function improvement in code-generation quality — a reminder that tool capability, not just hype, determines whether an AI-dependent business idea is currently viable.
- No user-level revenue verification is possible for most Bolt-built businesses since users own their own Stripe accounts — Eric's claimed monetization examples are anecdotal/self-reported, not audited figures.
