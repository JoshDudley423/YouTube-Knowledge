# The Most Useful AI Skill You Can Learn Today
Channel: The Koerner Office | https://www.youtube.com/watch?v=By9zAwSl3qg | Published: 2025-07-08

Interview with Michael Shimeles (developer/no-code builder, "Rasmic") on how non-technical people can start building AI-powered products/automations and finding profitable niches. Less a specific business case study, more a practical toolkit + niche-selection framework.

## Core niche-finding strategy: "start where you are, not in the tech bubble"
- Central advice: avoid competing in the crowded "tech bubble" (consumer AI wrapper apps, generic SaaS) where thousands of skilled engineers are already building — instead build for a specific existing industry/job you already have direct exposure to.
- Reasoning: solving a problem for an industry you're actually in gives you (1) deep, authentic knowledge of the real pain points, (2) built-in trust/credibility with that audience, and (3) an existing distribution channel (you already know and can talk to the customer) — described as "building it is part one; marketing it is a whole other behemoth," and being in the industry solves both.
- Concrete example given: a friend who works at a pharmacy is building tools specifically for pharmacies (annoying billing/compliance software) — not because he's a "technical genius" but because he has lived the pain points and knows the compliance requirements.
- Guest's own example: built a custom payment solution for his church because Canadian charity-payment software options were expensive/clunky and inflexible (couldn't just use Stripe directly) — says if he wanted to turn it into a business, it would likely succeed specifically because he lived the problem and could price it more fairly than existing options.
- Bookkeeping/tax example (a "free idea" floated in the interview): an AI-first bookkeeping app to fix the pain of QuickBooks/Xero-style tools misclassifying transactions — guest says he personally wouldn't build it himself despite wanting it, because he's not an actual bookkeeper/accountant and would only build something "surface level"; the opportunity belongs to someone who's actually worked as a bookkeeper/accountant and knows the real workflow pain points.
- Referenced comparable success pattern ("X, except AI-first"): Cal AI positioned itself as "MyFitnessPal, but AI-first" (MyFitnessPal cited as a $100M+/hundreds-of-millions-of-users company) — the framing is to take an existing large, proven category and rebuild it AI-native from the ground up, the way Instagram was "Facebook, but mobile-first."

## Practical prompt-based technique for surfacing problems in your own industry
- Use an AI deep-research tool: prompt it to scan Reddit, message boards, and social media for complaints/gripes specific to your industry (example prompt given: "I'm a pharmacist, list common pain points for other pharmacists").
- Iterative refinement loop: get ~20 initial results, pick the 7 that ring truest to your own experience, feed those back and ask for "20 more like these 7" — repeating this narrows toward 30-50 genuinely relevant problems.
- From that refined list, take the top 2-3, build simple prototype "wrappers" around them, and distribute informally to friends/coworkers in that industry to get real feedback before investing further.

## No-code / low-code building tools recommended for non-technical builders
- Named automation/agent-building platforms for non-coders to start experimenting: **n8n** and **Gumloop** — explicitly recommended as accessible entry points to learn how workflow chaining works without writing code.
- Challenge issued to viewers: build at least one automation after watching, even something trivial and personal (e.g., an automation that reads spam emails and has GPT respond with a joke) — framed as a low-stakes way to build the underlying skill/confidence before attempting something monetizable.
- **"Vibe coding + vibe automating"**: combine a no-code automation platform (n8n/Gumloop) with a vibe-coding tool (guest mentions Tempo as one example, "or any other vibe code tool of your choice") using **webhooks** to connect an automation's back-end logic to an actual user-facing web app/interface — described as the technique for turning a personal automation into something you can actually package and sell, rather than a one-off script only you use.
- Worked example of a real automation the guest built for himself: an AI pizza-ordering chatbot agent that interfaced with an unofficial (reverse-engineered, six-year-old, non-secure) Domino's API — collected order preferences via conversational back-and-forth, stored answers in a database, then called the API to place and pay for the order. Guest explicitly did not publish or maintain this because the API was unofficial/insecure for handling payments — cited as a cautionary example about payment security when using undocumented third-party APIs.

## LLM/model selection framework (a spectrum used to decide which AI model to use for what)
- **Claude**: preferred for anything code-generation-heavy, calculation-heavy, or "agentic"/complex-logic tasks; described as the most expensive of the three.
- **OpenAI (ChatGPT)**: preferred for consumer/B2C-facing personality-driven outputs (chatbots with a voice/tone, image generators, poem generators) — recommended as the safe general-purpose default for a beginner building their first automation, since it's cheaper than Claude and versatile enough for most needs.
- **Gemini**: described as the most cost-effective "all-star," capable of both coding-adjacent tasks (2.5 Pro, "not as good as Claude but pretty up there") and general tasks — recommended for beginners who are especially cost-sensitive.
- General cost guidance: don't over-optimize for API cost early on ("worry about cost once you've made more than a million dollars") — pick the tool that fits the type of app you're building, not the cheapest one, in the early building stage.
- Guest reports switching primary tools over time based on model releases (e.g., dropped Claude entirely when 3.7 underperformed, then fully returned to Claude once 4.0 released) — the takeaway is to stay flexible and re-evaluate as new models launch rather than being loyal to one vendor.
- Perplexity and Grok were both tried but dropped in favor of OpenAI's built-in deep research/search and X's native Grok summarization features respectively — guest doesn't see a compelling reason to pay for either as separate tools currently.

## UX/design as a competitive moat
- Central claim: in a world where AI makes building functional apps trivial, polished user experience/design becomes the actual differentiator ("UX becomes a moat," referred to as "the sauce").
- Example cited: Linear vs. Jira — same core ticketing/project-tracking function, but Linear's design/UX quality is framed as its entire competitive advantage over an older, uglier incumbent.
- Advises non-technical builders to deliberately develop "taste" by collecting and studying sites/apps they admire (not for their revenue, but for how they make you feel), citing design references like Apple's visual style, component libraries like Magic UI, and tools like Gumloop's own site design.
- Practical workflow for copying a visual design with AI: (1) screenshot the reference design, (2) feed it to a vibe-coding tool and ask it to replicate the look — expect only ~40% fidelity on the first pass (layout/structure/buttons roughly right, details wrong); (3) go section-by-section, screenshotting and calling out specific mismatches (e.g., "these buttons should be more rounded," "this background should have a gradient, not flat white") to iteratively close the gap to a near-exact match.
- Sequencing advice: for a complete beginner, build the landing page first for a motivating "dopamine hit" / momentum; for someone more experienced, build the actual product functionality first and treat visual design as a separate, later-focused pass, since attempting both design and functionality simultaneously is harder to manage well.
- Advises non-technical builders to cultivate at least a couple of developer friends/relationships (even informally, e.g. "buy them pizza") — cited a personal example of fixing a cousin's multi-day API integration blocker in 30 seconds — framed as a low-cost way to unblock technical dead-ends without hiring.

## MCP (Model Context Protocol) explained
- Analogy used: MCP is like "USB-C for LLMs" — a standard interface letting any LLM connect to any third-party service (e.g., Linear) without needing to write custom integration code against each service's unique API documentation.
- Without MCP, integrating an LLM with many different tools means learning and coding against each tool's separate API individually — MCP standardizes this so an LLM ("MCP client") can plug into any compliant third-party "MCP server" the same way.
- Framed as foundational infrastructure for building more robust, general-purpose AI assistants (the "Jarvis"-style assistant that can fetch email, manage tickets, and take actions across many services) rather than single-purpose bots.
- Caveat: MCP is still an evolving/imperfect standard at time of recording (with Anthropic and others still working out details), not a fully mature solution.

## Learning-path recommendation: MCP/n8n vs. simpler no-code tools
- For people with limited free time (1-2 hours/day): just become a heavy daily user of mainstream LLMs (e.g., default to ChatGPT before Googling) rather than investing in deeper technical tools like MCP/n8n.
- For people with more free time: experiment broadly across everything (Lindy, Gumloop, Zapier, n8n, MCP, etc.) since tools and best practices change quickly and hands-on experience compounds regardless of which specific tool ends up winning.
- Overall framing: "pessimists are often right, optimists make money" — the advice throughout favors bias-to-action and experimentation over waiting for certainty about which tools/models will win.

## Caveats / risks noted
- The Domino's pizza-ordering agent example is explicitly flagged as unsafe to productize (unofficial API, insecure payment handling) — presented as a learning example, not a replicable business.
- No specific revenue figures or business outcomes are reported in this episode for the guest's own or others' AI-tool-based businesses — the content is instructional/strategic rather than a case study with numbers.

## Tools/platforms named
- n8n, Gumloop — no-code automation/agent-building platforms recommended for beginners.
- Tempo — one example vibe-coding tool mentioned (guest says any similar tool works).
- Claude (Anthropic), ChatGPT/OpenAI, Gemini (Google) — primary LLMs discussed and compared; Perplexity and Grok mentioned but not recommended as must-use tools by the guest.
- Zapier, Lindy — mentioned as alternative automation tools for people with more time to explore.
- Magic UI — a component/design library referenced as a source of UI design inspiration.
- Linear (vs. Jira) — cited as the running example of UX-as-differentiator.
- MCP (Model Context Protocol) — Anthropic-originated standard for LLM-to-service integration, explained conceptually.
