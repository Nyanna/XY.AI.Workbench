#!/usr/bin/env bash
# ~/.claude/statusline.sh
# Zeigt: Kontext% | In-Tokens | Out-Tokens | 5h-Limit% (Xmin) | 7d-Limit% (Xmin)

input=$(cat)

# ── ANSI Farben ────────────────────────────────────────────────────────────────
R='\033[0m'          # Reset
GRN='\033[32m'       # Grün  (< 50%)
YLW='\033[33m'       # Gelb  (50–79%)
RED='\033[31m'       # Rot   (≥ 80%)
DIM='\033[2m'        # Gedimmt für Labels
CYN='\033[36m'       # Cyan  für Trennzeichen

SEP="${CYN} │${R}"

# ── Hilfsfunktion: Tokens lesbar formatieren ───────────────────────────────────
# 1234 → "1.2k", 1234567 → "1.2M", 999 → "999"
human_tokens() {
  local n=$1
  if   [ "$n" -ge 1000000 ]; then
    printf "%.1fM" "$(echo "scale=2; $n/1000000" | bc)"
  elif [ "$n" -ge 1000 ]; then
    printf "%.1fk" "$(echo "scale=2; $n/1000" | bc)"
  else
    printf "%s" "$n"
  fi
}

# ── Hilfsfunktion: Farbe je nach Prozentwert ──────────────────────────────────
pct_color() {
  local pct=$1
  if   [ "$pct" -ge 80 ]; then printf "%s" "$RED"
  elif [ "$pct" -ge 50 ]; then printf "%s" "$YLW"
  else                          printf "%s" "$GRN"
  fi
}

# ── Hilfsfunktion: Minuten bis reset_at (Unix-Epoch) ──────────────────────────
mins_until() {
  local epoch=$1
  local now
  now=$(date +%s)
  local diff=$(( epoch - now ))
  if [ "$diff" -le 0 ]; then
    printf "reset"
  else
    printf "%d min" $(( diff / 60 ))
  fi
}

# ── Felder aus JSON extrahieren ───────────────────────────────────────────────
CTX_PCT=$(echo "$input" | jq -r '.context_window.used_percentage // 0' | cut -d. -f1)
IN_TOK=$(echo  "$input" | jq -r '.context_window.total_input_tokens  // 0')
OUT_TOK=$(echo "$input" | jq -r '.context_window.total_output_tokens // 0')

HOUR_PCT=$(echo   "$input" | jq -r '.rate_limits.five_hour.used_percentage // empty' 2>/dev/null | cut -d. -f1)
HOUR_RST=$(echo   "$input" | jq -r '.rate_limits.five_hour.resets_at       // empty' 2>/dev/null)
DAY_PCT=$(echo    "$input" | jq -r '.rate_limits.seven_day.used_percentage // empty' 2>/dev/null | cut -d. -f1)
DAY_RST=$(echo    "$input" | jq -r '.rate_limits.seven_day.resets_at       // empty' 2>/dev/null)

# ── Segment 1: Kontext ─────────────────────────────────────────────────────────
C=$(pct_color "${CTX_PCT:-0}")
CTX_SEG="${DIM}ctx:${R} ${C}${CTX_PCT}%${R}"

# ── Segment 2: Tokens in / out ────────────────────────────────────────────────
IN_HR=$(human_tokens "${IN_TOK:-0}")
OUT_HR=$(human_tokens "${OUT_TOK:-0}")
TOK_SEG="${DIM}in:${R} ${IN_HR}${DIM} out:${R} ${OUT_HR}"

# ── Segment 3: 5h-Rate-Limit ──────────────────────────────────────────────────
if [ -n "$HOUR_PCT" ] && [ -n "$HOUR_RST" ]; then
  H=$(pct_color "${HOUR_PCT}")
  H_MIN=$(mins_until "${HOUR_RST}")
  HOUR_SEG="${DIM}5h:${R} ${H}${HOUR_PCT}%${R} ${DIM}↻ ${R}${H_MIN}"
else
  HOUR_SEG="${DIM}5h:${R} ${DIM}–${R}"
fi

# ── Segment 4: 7d-Rate-Limit ──────────────────────────────────────────────────
if [ -n "$DAY_PCT" ] && [ -n "$DAY_RST" ]; then
  D=$(pct_color "${DAY_PCT}")
  D_MIN=$(mins_until "${DAY_RST}")
  DAY_SEG="${DIM}7d:${R} ${D}${DAY_PCT}%${R} ${DIM}↻ ${R}${D_MIN}"
else
  DAY_SEG="${DIM}7d:${R} ${DIM}–${R}"
fi

# ── Ausgabe ────────────────────────────────────────────────────────────────────
printf "%b%b%b%b%b%b%b%b%b\n" \
  "$CTX_SEG" "$SEP" \
  "$TOK_SEG" "$SEP" \
  "$HOUR_SEG" "$SEP" \
  "$DAY_SEG" "$R"
