import os
from typing import Any
import gi
gi.require_version("Gtk", "3.0")

from lutris.database import games, categories
from lutris import settings

covers = os.listdir(settings.COVERART_PATH)
banners = os.listdir(settings.BANNER_PATH)
icons = os.listdir(settings.ICON_PATH)

def find_basename(haystack: list[str], needle: str):
    for filename in haystack:
        basename = filename[:filename.rfind(".")]
        if basename == needle:
            return filename
    return None

def find_img_paths(game_slug: str):
    cover = find_basename(covers, game_slug)
    if cover: cover = f"{settings.COVERART_PATH}/{cover}"

    banner = find_basename(banners, game_slug)
    if banner: banner = f"{settings.BANNER_PATH}/{banner}"

    # icon names are preceded by "lutris_"
    icon = find_basename(icons, f"lutris_{game_slug}")
    if icon: icon = f"{settings.ICON_PATH}/{icon}"

    return {
        "cover": cover,
        "banner": banner,
        "icon": icon
    }

def dict_to_game(d: dict[str, Any]):
    img_paths = find_img_paths(d["slug"])
    return {
        "id": d["id"],
        "name": d["name"],
        "slug": d["slug"],
        "playtime": d.get("playtime"), # hours represented as a float
        "last_played": d.get("lastplayed"), # date as a timestamp
        "cover": img_paths.get("cover"),
        "banner": img_paths.get("banner"),
        "icon": img_paths.get("icon"),
        "categories": categories.get_categories_in_game(d["id"]),
        "runner": d["runner"], # is always set IF the game is installed
        "run_command": f"lutris lutris:rungameid/{d['id']}",
    }

def get_games():
    xs: list[dict[str, Any]] = games.get_games(filters={"installed": 1})
    xs = list(map(dict_to_game, xs))
    return xs
