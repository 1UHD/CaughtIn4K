//MC COLORS
const black = "#000000";
const dark_blue = "#0000AA";
const dark_green = "#00AA00";
const dark_aqua = "#00AAAA";
const dark_red = "#AA0000";
const dark_purple = "#AA00AA";
const gold = "#FFAA00";
const gray = "#AAAAAA";
const dark_gray = "#555555";
const blue = "#5555FF";
const green = "#55FF55";
const aqua = "#55FFFF";
const red = "#FF5555";
const pink = "#FF55FF";
const yellow = "#FFFF55";
const white = "#FFFFFF"

const star_colors: any = {
    0: [gray],
    100: [white],
    200: [gold],
    300: [aqua],
    400: [dark_green],
    500: [dark_aqua],
    600: [dark_red],
    700: [pink],
    800: [blue],
    900: [dark_purple],
    1000: [red, gold, yellow, green, aqua, pink, dark_purple],
    1100: [gray, white, white, white, white, gray, gray],
    1200: [gray, yellow, yellow, yellow, yellow, gold, gray],
    1300: [gray, aqua, aqua, aqua, aqua, dark_aqua, gray],
    1400: [gray, green, green, green, green, dark_green, gray],
    1500: [gray, dark_aqua, dark_aqua, dark_aqua, dark_aqua, blue, gray],
    1600: [gray, red, red, red, red, dark_red, gray],
    1700: [gray, pink, pink, pink, pink, dark_purple, gray],
    1800: [gray, blue, blue, blue, blue, dark_blue, gray],
    1900: [gray, dark_purple, dark_purple, dark_purple, dark_purple, dark_gray, gray],
    2000: [dark_gray, gray, white, white, gray, gray, dark_gray],
    2100: [white, white, yellow, yellow, gold, gold, gold],
    2200: [gold, gold, white, white, aqua, dark_aqua, dark_aqua],
    2300: [dark_purple, dark_purple, pink, pink, gold, yellow, yellow],
    2400: [aqua, aqua, white, white, gray, gray, dark_gray],
    2500: [white, white, green, green, dark_green, dark_green, dark_green],
    2600: [dark_red, dark_red, red, red, pink, pink, dark_purple],
    2700: [yellow, yellow, white, white, dark_gray, dark_gray, dark_gray],
    2800: [green, green, dark_green, dark_green, gold, gold, yellow],
    2900: [aqua, aqua, dark_aqua, dark_aqua, blue, blue, blue],
    3000: [yellow, yellow, gold, gold, red, red, dark_red]
};

function formatToSpan(texts: string[], colors: string[]): any {
    return (
        <>
            {texts.map((item, index) => (
                <span key={index} style={{ color: colors[index] }}>
                    {item}
                </span>
            ))}
        </>
    );
}

export function formatStars(stars: number) {
    let prestige = Math.floor(stars/100)*100;
    let displaystars = "";

    if (prestige > 3000) {
        prestige = 3000;
    }

    if (prestige <= 1000) {
        displaystars = `[${stars}✫]`
    } else if (prestige > 1000 && prestige <= 2000) {
        displaystars = `[${stars}✪]`
    } else {
        displaystars = `[${stars}✪]`
    }

    if (prestige < 1000) {
        return formatToSpan([displaystars], star_colors[prestige]);
    } else {
        return formatToSpan(displaystars.split(""), star_colors[prestige]);
    }
}

function getRankColorHEX(rankcolor: any) {
    switch(rankcolor) {
        case "RED":
            return red;
        case "GOLD":
            return gold;
        case "GREEN":
            return green;
        case "YELLOW":
            return yellow;
        case "LIGHT_PURPLE":
            return pink;
        case "WHITE":
            return white;
        case "BLUE":
            return blue;
        case "DARK_GREEN":
            return dark_green;
        case "DARK_RED":
            return dark_red;
        case "DARK_AQUA":
            return dark_aqua;
        case "DARK_PURPLE":
            return dark_purple;
        case "DARK_GRAY":
            return dark_gray;
        case "BLACK":
            return black;
        case "DARK_BLUE":
            return dark_blue
        default:
            return gray;
    }
}

export function formatRank(
    rank: string | undefined,
    monthlyrank: string | undefined,
    staffrank: string | undefined,
    rankcolor: string | undefined,
    name: string
) {
    if (!rank || !monthlyrank || !staffrank || !rankcolor) {
        return formatToSpan(["[", "NICK", "] ", name], [gray, yellow, gray, gray])
    }

    let pluscolor = getRankColorHEX(rankcolor);

    let shownRank = "";

    if (staffrank && staffrank !== "NONE") {
        shownRank = staffrank;
    } else if (monthlyrank && monthlyrank !== "NONE") {
        shownRank = monthlyrank;
    } else if (rank && rank !== "NONE") {
        shownRank = rank;
    } else {
        shownRank = "NON";
    }

    switch (shownRank) {
        case "YOUTUBER":
            return formatToSpan(
                ["[", "Y", "O", "U", "T", "U", "B", "E", "] ", name],
                [
                    red,
                    white,
                    white,
                    white,
                    white,
                    white,
                    white,
                    white,
                    red,
                    red,
                ]
            );

        case "GAME_MASTER":
            return formatToSpan(["[GM] ", name], [dark_green, dark_green]);

        case "ADMIN":
            return formatToSpan(["[ADMIN] ", name], [red, red]);

        case "SUPERSTAR":
            return formatToSpan(
                ["[", "M", "V", "P", "+", "+", "] ", name],
                [
                    gold,
                    gold,
                    gold,
                    gold,
                    pluscolor,
                    pluscolor,
                    gold,
                    gold,
                ]
            );

        case "MVP_PLUS":
            return formatToSpan(
                ["[", "M", "V", "P", "+", "] ", name],
                [aqua, aqua, aqua, aqua, pluscolor, aqua, aqua]
            );

        case "MVP":
            return formatToSpan(["[MVP] ", name], [aqua, aqua]);

        case "VIP_PLUS":
            return formatToSpan(
                ["[", "V", "I", "P", "+", "] ", name],
                [
                    green,
                    green,
                    green,
                    green,
                    gold,
                    green,
                    green,
                ]
            );

        case "VIP":
            return formatToSpan(["[VIP] ", name], [green, green]);

        case "NON":
            return formatToSpan([name], [gray]);

        default:
            return formatToSpan(["[", "NICK", "] ", name], [gray, yellow, gray, gray]);
    }
}