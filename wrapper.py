from typing import Any
import gi
gi.require_version("Gtk", "3.0")

from lutris.database import games
from lutris import settings

def get_games():
    xs: list[dict[str, Any]] = games.get_games(filters={"installed": 1})
    xs = [
        {
            "id": x["id"],
            "name": x["name"],
            "slug": x["slug"],
            "playtime": x.get("playtime"), # hours represented as a float
            "last_played": x.get("lastplayed"), # date as a timestamp
            # TODO: This ones must probably be retrieved manually
            "cover": None,
            "banner": None,
            "icon": None,
            "run_command": None,
        }
        for x in xs
    ]

    return xs
