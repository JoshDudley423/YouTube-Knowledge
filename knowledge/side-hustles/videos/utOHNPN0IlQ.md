# The Laziest AI Side Hustle Just Got Even Easier
Channel: The Koerner Office | https://www.youtube.com/watch?v=utOHNPN0IlQ | Published: 2026-05-22

## Core idea: build a faceless YouTube channel end-to-end using Claude + Higgsfield (via MCP)
- Faceless channels (no camera, no on-screen host/team) claimed to earn $5,000-$30,000/month; built entirely with AI script writing, AI image/video generation, and AI voice.

## Four monetization paths for a faceless channel
1. **AdSense** — YouTube pays per 1,000 views; finance-niche channels earn ~$15-30 CPM, storytelling-niche channels ~$3-8 CPM. Even small channels in the right niche can at least "pay your rent."
2. **Affiliate links** — point audience to a tool/product, earn a commission on purchases.
3. **Selling your own product** — templates, physical goods, newsletter subscriptions — described as "where the real money is."
4. **Selling the channel itself** — a faceless channel earning $3,000/month in AdSense alone reportedly sells for ~$100,000 (roughly 3x annual profit) on marketplaces like BizBuySell, Flippa, and Acquire.com.

## Tooling / workflow setup
- **MCP (Model Context Protocol)** explained as a "power cord" that plugs an AI chat tool (Claude) into another tool (here, Higgsfield, an AI media-creation platform) so scripting, image generation, and video generation all happen inside one Claude chat window instead of copy-pasting between apps.
- Setup: in Claude, Settings > Connectors > Customize > Add custom connector > name it (e.g. "Higgsfield") > paste the Higgsfield MCP URL > Connect > Allow. Link provided by host: https://higgsfield.ai/s/mcp-thekoerneroffice-TBNiUZ.
- Other MCPs the host uses day-to-day for reference: newsletter analytics, Zoom transcript review, email drafting in his own voice/style, Stripe, Mercury (banking).

## Niche selection (5 recommended niches for AI faceless channels, ranked by trade-offs)
1. **Dark business history** (scams, collapsed companies, billionaire empires — "American Greed" style; can even have an AI agent surface obscure local news stories about local scammers no major outlet covered).
2. **Mythology and lost civilizations** — high view counts, visually well-suited to AI-generated video.
3. **AI tools and tech predictions** — lower view counts but much higher RPM (cited $5 vs. $25 per 1,000 views range).
4. **Weird science / quirky topics** — loyal audiences, good for newsletter conversion.
5. **True crime / cold cases** — massive demand, AI handles dramatic-recreation visuals well.
- Selection criteria: pick a niche that (a) pays well per view and (b) actually looks good rendered as AI video (not all topics do).

## Step-by-step production workflow demonstrated (niche used: dark business history)
1. Prompt Claude (using Opus model with "adaptive thinking," directed to use the Higgsfield MCP): "Act as a faceless YouTube strategist. My niche is dark business history. Give me a channel name, a 10-word description, 10 video title ideas built to get clicks, then write the full script for video one. Cinematic tone, 8 minutes long, built for retention." Push back on/iterate on Claude's output like an editor (make the opening punchier, cut boring sections) rather than accepting the first draft.
2. **Video length rule**: target over 8 minutes — YouTube only allows mid-roll ads (in addition to pre-/post-roll) on videos 8+ minutes long, directly increasing ad revenue per video.
3. Generate a clickbait-style thumbnail via the Higgsfield MCP's image generation (or explicitly request ChatGPT's "Image 2" model instead, which the host says handles on-image text noticeably better — ~10-30% better result for a few cents of extra credit cost); add bold "hook" text like "THEY FOOLED YOU."
4. Prompt Claude to break the finished script into 12 cinematic shots, write a detailed visual prompt per shot, and generate each via Higgsfield (host specified "dark, cinematic, slow camera movement" for visual consistency). Higgsfield uses an underlying video model called "Seed Dance 2.0"; each generated clip takes roughly 30-60 seconds to render, and clips can be generated in parallel.
5. **Face Lock feature** (Higgsfield): upload one reference face (AI-generated, real, or an avatar) and the tool keeps that same face consistent across every shot/scene in the video — lets a "faceless" channel still feature a recurring on-screen character without ever using the creator's own face.
6. **Voice options**: Higgsfield Speak 2.0 (write script text with ALL CAPS for emphasis and "..." for pauses, which the tool interprets) or ElevenLabs (can clone your own or someone else's voice — host uses ElevenLabs to dub his own videos into other languages in his own cloned voice).
7. **Assembly**: import all generated video clips into CapCut (free, works on desktop/iPhone/Android), lay the voiceover track over them, add royalty-free background music.
8. **Upload/optimization**: YouTube Studio > Create > Upload; use YouTube's built-in A/B testing feature to test up to 3 different titles/thumbnails simultaneously — YouTube shows each version to different viewers and reports click-through rates, then auto-selects (or lets you manually pick) a winner. Thumbnails can be AI-generated directly inside YouTube too.
- Host also demoed a "predict virality" feature in Higgsfield on an individual generated shot, though he notes it's of questionable relevance for a single clip within a longer video rather than a standalone short-form post.

## Cost/economics comparison: AI workflow vs. traditional production
- Traditional cost estimate per video (hiring humans): writer $200-400, editor $300-500, voice actor $150-300, thumbnail designer ~$50 (more if A/B testing multiple thumbnails) — totaling roughly $1,000-1,500 per video with no guarantee of performance. Producing 24 videos (enough to likely find some "winners") the traditional way would cost an estimated $30,000-35,000, out of reach for most people.
- AI-tool workflow instead costs "a couple subscriptions and some credits," collapsing what used to take 5-20 hours of work (for someone with no video editing experience) into about an hour per video.
- Revenue potential cited: a single video that reaches 100,000 views can generate $500-$2,000 depending on niche — described as unlikely on video 1 or even video 10, more of a "numbers game" that pays off with enough volume and iterative improvement.
- Fallback monetization if your own channel doesn't take off: offer this same AI video creation workflow as a paid service to other creators/businesses.

## Tools/platforms named
- Claude (specifically using an Opus model with "adaptive thinking") — scripting/strategy brain.
- Higgsfield — AI media generation platform (images, video via "Seed Dance 2.0," voice via "Speak 2.0," Face Lock consistency feature), connected to Claude via MCP.
- ChatGPT "Image 2" model — alternate thumbnail generator, noted as superior for rendering text on images.
- ElevenLabs — voice cloning/dubbing.
- CapCut — free video editor (desktop/mobile) for final assembly, music, voiceover sync.
- Marketplaces for selling a built-out channel: BizBuySell, Flippa, Acquire.com.
- A companion Google Doc of all prompts used in the video was published by the host (linked in video description).

## Caveats
- The full prompt library/workflow shown is presented as directly copyable regardless of niche chosen, but actual monetization results (view counts, RPMs, virality) are explicitly framed as unpredictable and volume-dependent, not guaranteed by following the workflow alone.
- Sponsor/referral relationship disclosed: video includes an affiliate link/referral code to Higgsfield's MCP.
