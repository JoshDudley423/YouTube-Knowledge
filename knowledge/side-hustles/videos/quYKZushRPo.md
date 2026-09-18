# The Beginner-Friendly Claude AI Side Hustle Nobody Talks About
Channel: The Koerner Office | https://www.youtube.com/watch?v=quYKZushRPo | Published: 2026-05-13

Interview with Ryan Dozer (ryandozer.com), a non-technical AI marketer/SEO consultant, on building and selling "Claude skills" as a digital product/side hustle.

## The core idea
- A "Claude skill" = a standard operating procedure (SOP) file (markdown) that tells an AI coding agent (Claude Code, Codex, etc.) exactly how to do a specific repeatable task — described as "a recipe" or "a mini superpower." Best practice: keep skills narrow/granular (one skill per specific subtask — e.g., a dedicated SEO blog post writer skill, a separate email newsletter writer skill, a separate sales email skill) rather than one broad skill.
- Framed as an underexploited app-store-style opportunity, analogous to (but far less crowded than) the Apple App Store, Shopify App Store, Chrome Extension Store, WordPress plugins, or Notion template marketplaces.
- Key insight/pitch: anyone with subject-matter expertise in *anything* (marketing, gardening, nonprofit work, trading Pokémon or baseball cards, investing) can package that expertise into a skill file and sell it, without needing to code — you build it by voice-dictating what you know into Claude and using Anthropic's free "skill creator" skill (available on Anthropic's GitHub) to convert your knowledge/existing Claude "Projects" into a proper skill markdown file.

## Concrete numbers (Ryan's own product)
- Product: "Claude Code Skill Stack" — a bundle of ~20-25 individual skills — launched ~30-45 days before this interview.
- Revenue: over $3,000 in that window, described as "passive," plus at least one $1,000 consulting upsell (a buyer was impressed by a skill and paid $1,000 for an hour of Ryan's time on the spot).
- Price point: sells the full skill stack for $99.
- Distribution: sold entirely through his own website/landing page (not through any official Claude/Anthropic marketplace) with a Stripe payment link; promoted via a floating pop-up, blog post CTAs, and email newsletter drip sequences — described as a "soft sell," not aggressively pushed.
- His own background: runs a six-figure one-person marketing agency; first client (2019, SEO/content marketing) paid ~$10,000/month.

## How to build and package a skill (step-by-step, as demonstrated)
1. Start from an existing Claude "Project" you already use regularly for a specific task (Ryan showed his own examples: a yard-care Q&A project, a YouTube video intro-writing project, a podcast-retention-data analysis project, an "email like me" writing-style project, etc.).
2. Install/use Anthropic's official "skill creator" skill (rather than a raw "create a skill" prompt) for best-practice structure.
3. Prompt it (voice-dictation is fine): "Use the skill creator skill to transform this project into a skill markdown file, keep the instructions/memory, review my recent chat history..." — produces a downloadable `.md` skill file.
4. That markdown file can be used directly inside Claude, transferred to another AI coding agent (Codex, Gemini) if you switch tools, or uploaded as project knowledge in another AI chat product — skills are explicitly described as portable/tool-agnostic, unlike a Claude "Project" which doesn't transfer.
5. Skills can self-update: if you're using a skill (e.g., writing an article) and something is off, telling the AI to fix it can update the skill definition in place so it won't repeat that mistake — can be pushed further into fully automatic self-updating (referenced to Andrej Karpathy's "auto research" concept), though this is more advanced.
6. To package for sale: bundle multiple related skills into one product, build a landing page (Ryan used his own "web designer" skill, fed it his brand/logo/tone, and had Claude Code "vibe code" the landing page in ~15-20 minutes), and connect a Stripe payment link.
7. Advise buyers/users to always customize a downloaded skill with their own context/style rather than using it as-is, so outputs match their voice, not the original creator's.

## Named high-value example skills in Ryan's own stack (the ~80/20 of what people actually use)
- **SEO blog post writer**: converts a YouTube video URL into a full SEO-optimized blog post (with internal/external linking, keyword targeting, XML sitemap awareness) in roughly one Claude Code prompt; demonstrated live producing a ranking blog post from a recent YouTube video with no further edits, credited with getting his content to appear in Google AI Overviews and organic search alongside the original YouTube video.
- **YouTube thumbnail designer**: uses GPT Image models to generate thumbnails.
- **"Anti-slop" skill**: strips out AI-cliché phrasing ("not this, but that," "delve," "robust," etc.) from generated content.
- Other named use cases: a LinkedIn/social profile scraper skill (paired with Apify) that pulls a person's recent posts into a spreadsheet with engagement scores to analyze what content performs; a personal "AI marketing OS" dashboard he vibe-coded that syncs Google Calendar and a social content calendar and connects (via MCP) to a social scheduling tool (Blotato) so he never has to leave the Claude Code interface.

## Distribution/marketing model notes
- No official Claude/Anthropic marketplace for skills exists (as of this recording) — sellers use their own website + payment processor.
- Recommends `.md` (markdown) or `.txt` files over PDFs when feeding large amounts of context to an AI agent — claims AI models parse markdown and HTML far more reliably/efficiently than PDFs, which "miss things."
- Suggested progression for viewers: stop using ad-hoc ChatGPT/Claude/Gemini chat sessions and "Projects" (copy-paste workflows), start building skills, then move into an agentic coding tool (Claude Code or Codex) to run those skills directly rather than manually copy-pasting between chat windows.

## Caveats
- Described as non-technical-friendly, but does still require using Claude Code/an agentic coding interface (a step up from basic chat) to get the full workflow benefits shown (e.g., auto-generating a landing page, running the SEO blog skill via a Claude Code/VS Code extension).
- Revenue is from a brand-new site/product (a few months old at recording) — traffic and rankings were still growing, not yet mature/plateaued.
- No data given on refund rates, churn, or whether repeat/ongoing revenue exists beyond the initial $3,000+ launch window.
