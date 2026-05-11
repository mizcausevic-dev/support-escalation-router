from pathlib import Path
from PIL import Image, ImageDraw, ImageFont


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "screenshots"
OUT.mkdir(exist_ok=True)

WIDTH = 1440
HEIGHT = 900
BG = "#091321"
PANEL = "#131f31"
CARD = "#1a2940"
BORDER = "#294764"
TEXT = "#f3efe1"
SUB = "#b8c7dc"
ACCENT = "#8bc5ff"
WARN = "#ffd76d"


def font(size: int, bold: bool = False):
    candidates = [
        "C:/Windows/Fonts/seguisb.ttf" if bold else "C:/Windows/Fonts/segoeui.ttf",
        "C:/Windows/Fonts/arialbd.ttf" if bold else "C:/Windows/Fonts/arial.ttf",
    ]
    for candidate in candidates:
        path = Path(candidate)
        if path.exists():
            return ImageFont.truetype(str(path), size)
    return ImageFont.load_default()


TITLE = font(54, True)
SECTION = font(18, False)
BODY = font(26, False)
CARD_TITLE = font(18, False)
CARD_VALUE = font(34, True)
CARD_BODY = font(19, False)


def wrapped(draw, text, xy, wrap_width, font_obj, fill, line_gap=10):
    words = text.split()
    lines = []
    current = ""
    for word in words:
        trial = word if not current else f"{current} {word}"
        if draw.textlength(trial, font=font_obj) <= wrap_width:
            current = trial
        else:
            lines.append(current)
            current = word
    if current:
        lines.append(current)
    x, y = xy
    line_height = font_obj.size + line_gap
    for line in lines:
        draw.text((x, y), line, font=font_obj, fill=fill)
        y += line_height
    return y


def shell():
    image = Image.new("RGB", (WIDTH, HEIGHT), BG)
    draw = ImageDraw.Draw(image)
    draw.rounded_rectangle((30, 30, WIDTH - 30, HEIGHT - 30), 28, fill=PANEL, outline=BORDER, width=2)
    return image, draw


def hero():
    image, draw = shell()
    draw.text((90, 82), "SUPPORT ESCALATION ROUTER", font=SECTION, fill=ACCENT, spacing=4)
    wrapped(draw, "Route support pressure into the owner lane that can actually absorb it.", (90, 130), 1160, TITLE, TEXT, 8)
    wrapped(draw, "Queue posture, SLA compression, callback risk, and incident handoffs in one Rust service.", (90, 270), 980, BODY, SUB, 8)

    cards = [
        ("OPEN THREADS", "3", "Active multi-lane support incidents"),
        ("ESCALATE NOW", "2", "SLA windows inside 30 minutes"),
        ("OWNER LANES", "4", "Billing, identity, growth, incident"),
        ("CALLBACK RISK", "High", "Enterprise confidence under strain"),
    ]
    x = 90
    for title, value, body in cards:
        draw.rounded_rectangle((x, 360, x + 285, 555), 22, fill=CARD, outline=BORDER, width=2)
        draw.text((x + 24, 388), title, font=CARD_TITLE, fill=SUB)
        draw.text((x + 24, 430), value, font=CARD_VALUE, fill=TEXT)
        wrapped(draw, body, (x + 24, 488), 230, CARD_BODY, SUB, 6)
        x += 305

    draw.rounded_rectangle((90, 610, WIDTH - 90, 800), 24, fill=CARD, outline=BORDER, width=2)
    draw.text((120, 642), "CURRENT DECISION LANE", font=CARD_TITLE, fill="#ffbfdc")
    wrapped(draw, "Freeze secondary callbacks and move the enterprise billing export into incident-command ownership before finance close.", (120, 684), 1120, font(42, True), TEXT, 8)
    image.save(OUT / "01-hero.png")


def lanes():
    image, draw = shell()
    draw.text((90, 82), "QUEUE LANES", font=SECTION, fill=ACCENT)
    wrapped(draw, "Support routing should be visible as a set of clean lanes, not a blur of ticket ownership.", (90, 130), 1180, TITLE, TEXT, 8)
    positions = [
        (90, 300, "BILLING PLATFORM", "Critical export lanes", "SLA 18m · 4 handoffs"),
        (460, 300, "IDENTITY SYSTEMS", "Regional SSO failover", "SLA 42m · 3 handoffs"),
        (830, 300, "GROWTH SYSTEMS", "Launch-day trial routing", "SLA 64m · 2 handoffs"),
    ]
    for x, y, title, body, stat in positions:
        draw.rounded_rectangle((x, y, x + 300, y + 240), 22, fill=CARD, outline=BORDER, width=2)
        draw.text((x + 24, y + 28), title, font=CARD_TITLE, fill=ACCENT)
        wrapped(draw, body, (x + 24, y + 78), 240, font(36, True), TEXT, 8)
        draw.text((x + 24, y + 180), stat, font=font(24, False), fill=WARN)
    draw.rounded_rectangle((90, 590, WIDTH - 90, 800), 24, fill=CARD, outline=BORDER, width=2)
    draw.text((120, 624), "WHY IT MATTERS", font=CARD_TITLE, fill="#ffbfdc")
    wrapped(draw, "The same service can decide whether a thread stays with a queue lead, jumps to a systems owner, or escalates into incident command.", (120, 672), 1140, font(32, True), TEXT, 8)
    image.save(OUT / "02-queue-lanes.png")


def escalation_detail():
    image, draw = shell()
    draw.text((90, 82), "ESCALATION DETAIL", font=SECTION, fill=ACCENT)
    wrapped(draw, "One thread, one route decision, one immediate next action.", (90, 130), 1080, TITLE, TEXT, 8)
    draw.rounded_rectangle((90, 290, 770, 780), 24, fill=CARD, outline=BORDER, width=2)
    draw.text((120, 322), "SUP-9104 · ENTERPRISE BILLING EXPORT", font=CARD_TITLE, fill=ACCENT)
    wrapped(draw, "Status: escalate", (120, 382), 500, font(38, True), TEXT, 6)
    draw.text((120, 450), "Recommended lane: incident-command", font=font(24, False), fill=SUB)
    draw.text((120, 494), "Score: 100", font=font(24, False), fill=SUB)
    draw.text((120, 552), "Key risks", font=font(26, True), fill=TEXT)
    wrapped(draw, "SLA breach risk, ownership diffusion, callback strain, and export freeze pressure.", (120, 594), 560, CARD_BODY, SUB, 6)

    draw.rounded_rectangle((810, 290, WIDTH - 90, 780), 24, fill=CARD, outline=BORDER, width=2)
    draw.text((840, 322), "IMMEDIATE ACTION", font=CARD_TITLE, fill="#ffbfdc")
    wrapped(draw, "Assign a single escalation owner, freeze secondary callbacks, and route the export lane into incident command now.", (840, 382), 480, font(30, True), TEXT, 8)
    draw.text((840, 560), "Stabilizers", font=font(26, True), fill=TEXT)
    wrapped(draw, "Specialist lane already exists. Next-step sequence is defined. Recovery path is known once ownership is collapsed.", (840, 604), 470, CARD_BODY, SUB, 6)
    image.save(OUT / "03-escalation-detail.png")


def proof():
    image, draw = shell()
    draw.text((90, 82), "VALIDATION PROOF", font=SECTION, fill=ACCENT)
    wrapped(draw, "API surface, route analysis, and cargo validation are all visible in one proof layer.", (90, 130), 1180, TITLE, TEXT, 8)
    draw.rounded_rectangle((90, 300, 760, 790), 24, fill="#07101c", outline=BORDER, width=2)
    proof_lines = [
        "> cargo test",
        "test critical_threads_escalate ... ok",
        "test low_pressure_threads_stay_stable ... ok",
        "test root_route_returns_ok ... ok",
        "test missing_ticket_returns_404 ... ok",
        "",
        "> POST /api/analyze/route",
        "{",
        '  "status": "escalate",',
        '  "recommended_lane": "incident-command"',
        "}",
    ]
    y = 336
    mono = font(24, False)
    for line in proof_lines:
        draw.text((120, y), line, font=mono, fill="#c8f7a5" if line.startswith(">") else SUB)
        y += 34

    draw.rounded_rectangle((810, 300, WIDTH - 90, 790), 24, fill=CARD, outline=BORDER, width=2)
    draw.text((840, 332), "PROOF POINTS", font=CARD_TITLE, fill=ACCENT)
    wrapped(draw, "The router shows queue posture, analyzes individual threads, and produces a human-usable owner recommendation with no front-end dependency.", (840, 390), 470, font(28, True), TEXT, 8)
    wrapped(draw, "That makes the repo useful both as a backend artifact and as a command surface input for a future escalation console.", (840, 575), 470, CARD_BODY, SUB, 6)
    image.save(OUT / "04-proof.png")


if __name__ == "__main__":
    hero()
    lanes()
    escalation_detail()
    proof()
