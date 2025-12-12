#!/bin/sh

if [ -z "$1" ]; then
    echo "Usage: $0 <leaderboard_id> [update] [day_number]"
    exit 1
fi

LEADERBOARD_ID="$1"
UPDATE_FLAG=""
DAY_ARG=""

shift
for arg in "$@"; do
    case "$arg" in
        update)
            UPDATE_FLAG="update"
            ;;
        ''|*[!0-9]*)
            echo "Unknown argument: $arg"
            exit 1
            ;;
        *)
            DAY_ARG="$arg"
            ;;
    esac
done

# Validate day argument if supplied
if [ -n "$DAY_ARG" ]; then
    if ! echo "$DAY_ARG" | grep -Eq '^[0-9]+$'; then
        echo "Error: day_number must be numeric."
        exit 1
    fi
fi

YEAR=2025
URL="https://adventofcode.com/$YEAR/leaderboard/private/view/$LEADERBOARD_ID.json"
COOKIE_FILE="aoc_cookie"

DIR="leaderboards"
FILE="$DIR/$LEADERBOARD_ID.json"

mkdir -p "$DIR"

download() {
    if [ ! -f "$COOKIE_FILE" ]; then
        echo "ERROR: session file '$COOKIE_FILE' not found."
        exit 1
    fi

    COOKIE=$(cat "$COOKIE_FILE")

    echo "Downloading $URL..."
    curl -s -H "Cookie: $COOKIE" -o "$FILE" "$URL" || {
        echo "Error: curl failed"
        exit 1
    }
}

if [ "$UPDATE_FLAG" = "update" ] || [ ! -f "$FILE" ] || find "$FILE" -mmin +15 -type f | grep -q .; then
    download
fi

NUM_DAYS=$(jq -r '.num_days // 25' "$FILE")

pretty_ts() {
    TS="$1"
    if [ "$TS" = "-" ]; then
        echo "-"
    else
        date -d "@$TS" "+%Y-%m-%d %H:%M:%S"
    fi
}

START_DAY=1
END_DAY="$NUM_DAYS"

if [ -n "$DAY_ARG" ]; then
    if [ "$DAY_ARG" -gt "$NUM_DAYS" ] || [ "$DAY_ARG" -lt 1 ]; then
        echo "Error: day_number $DAY_ARG is out of range (1-$NUM_DAYS)."
        exit 1
    fi
    START_DAY="$DAY_ARG"
    END_DAY="$DAY_ARG"
fi

DAY="$START_DAY"
while [ "$DAY" -le "$END_DAY" ]; do

    HAS_COMPLETIONS=$(jq -r --arg d "$DAY" '
        .members
        | to_entries
        | any(.value.completion_day_level[$d] != null)
    ' "$FILE")

    if [ "$HAS_COMPLETIONS" != "true" ]; then
        DAY=$((DAY + 1))
        continue
    fi

    echo "Day $DAY"
    for PART in 1 2; do
        echo "  Part $PART"

        # Extract: timestamp, member name
        # Format:  <timestamp> <name>
        jq -r --arg d "$DAY" --arg p "$PART" '
            .members
            | to_entries
            | map({
                name: .value.name,
                ts: (.value.completion_day_level[$d][$p].get_star_ts // null)
              })
            | map(select(.ts != null))
            | sort_by(.ts)
            | .[] | "\(.ts) \(.name)"
        ' "$FILE" |

        awk -v day="$DAY" -v part="$PART" '
            {
                ts = $1
                $1 = ""             # remove raw timestamp from $0
                name = substr($0,2) # strip leading space
                cmd = "date -d \"@" ts "\" \"+%Y-%m-%d %H:%M:%S\""
                cmd | getline pretty
                close(cmd)
                ++pos
                printf("    %-4s  %s  %s\n", pos, pretty, name)
            }
        '

        echo
    done

    DAY=$((DAY + 1))
done
