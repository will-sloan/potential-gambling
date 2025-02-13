import * as wasm from "wasm-poker-game";

var current_pos = 0;
function drawMainTable(ctx, x1, y1, x2, y2, r) {
    // Called pill shaped


    // ctx.fillStyle = "rgba(0,100,0,0.5)";
    // ctx.fillRect(x1, y1, x2, y2);
    //console.log(width);//, height * 0.1, width * 0.8, height * 0.8);
    ctx.beginPath();
    // start at top corner of rectangle
    ctx.moveTo(x1 + r, y1);
    // draw top line until spot where a curve should be
    ctx.lineTo(x2 + x1 - r, y1);
    // curve, set the control point to the rectangle corner and the second point is where to end up, which is yAtCorner - radius
    ctx.quadraticCurveTo(x2 + x1, y1, x2 + x1, y1 + r);
    // draw from where the curve finished, line straight down until next curve should start
    //ctx.moveTo(x2 + x1, y1 + r);
    ctx.lineTo(x2 + x1, y1 + y2 - r);

    // first two arguments is the bottom right corner of the rectangle and the next two is where the curve should end
    // ctx.quadraticCurveTo(x2 + x1, y2 + y1, x2 + x1 - r, y1 + y2);
    // ctx.lineTo(x1 + r, y2 + y1);
    // ctx.quadraticCurveTo(x1, y2 + y1, x1, y1 + y2 - r);
    // // ctx.lineTo(x1, y1 + r)
    // ctx.quadraticCurveTo(x1, y1, x1 + r, y1);
    ctx.quadraticCurveTo(x2 + x1, y2 + y1, x2 + x1 - r, y1 + y2);
    ctx.lineTo(x1 + r, y2 + y1);
    ctx.quadraticCurveTo(x1, y2 + y1, x1, y1 + y2 - r);
    ctx.lineTo(x1, y1 + r)
    ctx.quadraticCurveTo(x1, y1, x1 + r, y1);
    ctx.fillStyle = "rgba(0,100,0,0.5)";
    ctx.fill();
    //ctx.stroke();
}

function draw_next_button() {
    var buttonX = 70;
    var buttonY = 80;
    var buttonW = 60;
    var buttonH = 30;
    // Render button
    ctx.fillStyle = 'red';
    ctx.fillRect(buttonX, buttonY, buttonW, buttonH);
    canvas.addEventListener('click', function (event) {
        // Control that click event occurred within position of button
        // NOTE: This assumes canvas is positioned at top left corner
        // Not fucking sure how it works
        //console.log(event.x);
        //console.log(event.y);
        const rect = canvas.getBoundingClientRect()
        const x = event.clientX - rect.left
        const y = event.clientY - rect.top
        //console.log("x: " + x + " y: " + y)
        if (
            x > buttonX &&
            x < buttonX + buttonW &&
            y > buttonY &&
            y < buttonY + buttonH
        ) {
            // Executes if button was clicked!
            // alert('Button was clicked!');
            current_pos++;
            console.log("Button was clicked. Pos now: " + current_pos);
            m();
        }
    });
}

function redraw_button() {
    var buttonX = 70;
    var buttonY = 80;
    var buttonW = 60;
    var buttonH = 30;
    // Render button
    ctx.fillStyle = 'red';
    ctx.fillRect(buttonX, buttonY, buttonW, buttonH);
}
const card_height = 125;
const card_width = 75;
const p6andp3_similar_offset = 15;
function draw_players(game, ctx, x1, y1, x2, y2, r) {
    // Top Two
    // ctx.beginPath();
    // // Loc of Player 1 or the topish left
    // ctx.arc(x1 + x2 / 3, y1, 4, 0, Math.PI * 2);
    // ctx.stroke();
    if (game["players"].length >= 1) {
        var p1_c1 = new Image();
        p1_c1.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p1_c1, x1 + x2 / 3, y1, card_width, card_height);
        }
        p1_c1.src = game["players"][0]["cards"][0]["link"];

        var p1_c2 = new Image();
        p1_c2.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p1_c2, x1 + x2 / 3 - card_width, y1, card_width, card_height);
        }
        p1_c2.src = game["players"][0]["cards"][1]["link"];
    }


    // ctx.beginPath();
    // // Loc of Player 2 or the topish right
    // ctx.arc(x1 + x2 * 2 / 3, y1, 4, 0, Math.PI * 2);
    // ctx.stroke();

    if (game["players"].length >= 2) {
        var p2_c1 = new Image();
        p2_c1.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p2_c1, x1 + x2 * 2 / 3, y1, card_width, card_height);
        }
        p2_c1.src = game["players"][1]["cards"][0]["link"];

        var p2_c2 = new Image();
        p2_c2.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p2_c2, x1 + x2 * 2 / 3 - card_width, y1, card_width, card_height);
        }

        p2_c2.src = game["players"][1]["cards"][1]["link"];
    }


    // Loc of Player 3 or the right side
    // ctx.beginPath();
    // ctx.arc(x2 + x1, y2 / 2 + y1, 4, 0, Math.PI * 2);
    // ctx.stroke();
    if (game["players"].length >= 3) {
        var p3_c1 = new Image();
        p3_c1.onload = function () {
            //ctx.rotate();
            // 90 instead of 75 so they are completely on board
            ctx.drawImage(p3_c1, x2 + x1 - p6andp3_similar_offset - 75, y2 / 2 + y1, card_width, card_height);
        }

        p3_c1.src = game["players"][2]["cards"][0]["link"];

        var p3_c2 = new Image();
        p3_c2.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p3_c2, x2 + x1 - p6andp3_similar_offset - 75, y2 / 2 + y1 - card_height, card_width, card_height);
        }

        p3_c2.src = game["players"][2]["cards"][1]["link"];
    }

    // ctx.beginPath();
    // // Loc of Player 4 or the bottomish right
    // ctx.arc(x1 + x2 * 2 / 3, y1 + y2, 4, 0, Math.PI * 2);
    // ctx.stroke();
    if (game["players"].length >= 4) {
        var p4_c1 = new Image();
        p4_c1.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p4_c1, x1 + x2 * 2 / 3, y1 + y2 - card_height, card_width, card_height);
        }

        p4_c1.src = game["players"][3]["cards"][0]["link"];

        var p4_c2 = new Image();
        p4_c2.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p4_c2, x1 + x2 * 2 / 3 - card_width, y1 + y2 - card_height, card_width, card_height);
        }

        p4_c2.src = game["players"][3]["cards"][1]["link"];
    }

    // Bottom two
    // ctx.beginPath();
    // // Loc of Player 5 or the bottomish left
    // ctx.arc(x1 + x2 / 3, y1 + y2, 4, 0, Math.PI * 2);
    // ctx.stroke();
    if (game["players"].length >= 5) {
        var p5_c1 = new Image();
        p5_c1.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p5_c1, x1 + x2 / 3, y1 + y2 - card_height, card_width, card_height);
        }

        p5_c1.src = game["players"][4]["cards"][0]["link"];

        var p5_c2 = new Image();
        p5_c2.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p5_c2, x1 + x2 / 3 - card_width, y1 + y2 - card_height, card_width, card_height);
        }

        p5_c2.src = game["players"][4]["cards"][1]["link"];
    }

    // ctx.beginPath();
    // // Loc of Player 6 or the player of left side
    // ctx.arc(x1, y2 / 2 + y1, 4, 0, Math.PI * 2);
    // ctx.stroke();
    // ctx.clearRect(x1, y1, 50, 50);
    // ctx.save();
    // ctx.translate(x1 - 45, y2 / 2 + y1 - 60);
    // ctx.font = '36px serif';
    // ctx.rotate(Math.PI / 2);
    // ctx.fillText('Player 6', 0, 0);
    // ctx.restore();
    if (game["players"].length >= 6) {
        var p6_c1 = new Image();
        p6_c1.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p6_c1, x1 + p6andp3_similar_offset, y2 / 2 + y1, card_width, card_height);
        }

        p6_c1.src = game["players"][5]["cards"][0]["link"];

        var p6_c2 = new Image();
        p6_c2.onload = function () {
            //ctx.rotate();
            ctx.drawImage(p6_c2, x1 + p6andp3_similar_offset, y2 / 2 + y1 - card_height, card_width, card_height);
        }

        p6_c2.src = game["players"][5]["cards"][1]["link"];
    }
    //ctx.beginPath();
    // var p6_c1 = new Image();
    // p1_c1.onload = function () {
    //     //ctx.rotate();
    //     ctx.drawImage(p6_c1, x1, y2 / 2 + y1, 75, 125);
    // }
    // console.log("Here: ");

    // console.log(game["players"][0]["cards"][0]["link"]);

    // p6_c1.src = game["players"][0]["cards"][0]["link"];
}

function draw_names_and_scores(game, ctx, x1, y1, x2, y2, r) {
    // player 1
    if (game["players"].length >= 1) {
        ctx.font = '30px serif';
        //ctx.fontStyle = "aqua";
        ctx.fillText(game["players"][0]["ip"], x1 + x2 / 3 - card_width / 2, y1 - 55);
        ctx.fillText(game["players"][0]["handvalue"], x1 + x2 / 3 - card_width / 2, y1 - 15);
    }
    // player 2
    if (game["players"].length >= 2) {
        ctx.font = '30px serif';
        //ctx.fontStyle = "aqua";
        ctx.fillText(game["players"][1]["ip"], x1 + x2 * 2 / 3 - card_width / 2, y1 - 55);
        ctx.fillText(game["players"][1]["handvalue"], x1 + x2 * 2 / 3 - card_width / 2, y1 - 15);
    }
    // player 3
    if (game["players"].length >= 3) {
        ctx.font = '30px serif';
        //ctx.fontStyle = "aqua";
        ctx.fillText(game["players"][2]["ip"], x2 + x1, y2 / 2 + y1);
        ctx.fillText(game["players"][2]["handvalue"], x2 + x1, y2 / 2 + y1 + 30);
    }
    // player 4
    if (game["players"].length >= 4) {
        ctx.font = '30px serif';
        //ctx.fontStyle = "#00ff00";
        ctx.fillText(game["players"][3]["ip"], x1 + x2 * 2 / 3 - card_width / 2, y1 + y2 + 30);
        ctx.fillText(game["players"][3]["handvalue"], x1 + x2 * 2 / 3 - card_width / 2, y1 + y2 + 30 + 30);
    }
    // player 5
    if (game["players"].length >= 5) {
        ctx.stroke();
        ctx.font = '30px serif';
        //canvas.fontStyle = "#000000";
        ctx.fillText(game["players"][4]["ip"], x1 + x2 / 3 - card_width / 2, y1 + y2 + 30);
        ctx.fillText(game["players"][4]["handvalue"], x1 + x2 / 3 - card_width / 2, y1 + y2 + 30 + 30);
    }
    // player 6
    if (game["players"].length >= 6) {
        ctx.font = '30px serif';
        //ctx.fontStyle = "#00ff00";
        ctx.fillText(game["players"][5]["ip"], x1 - card_width * 1.3, y2 / 2 + y1);
        ctx.fillText(game["players"][5]["handvalue"], x1 - card_width * 1.3, y2 / 2 + y1 + 30);
    }
}

function draw_flop(game, ctx, x1, y1, x2, y2, r) {
    console.log("In flop");
    ctx.beginPath();
    // Loc of Player 6 or the player of left side
    ctx.arc(x2 / 2 + x1, y2 / 2 + y1, 4, 0, Math.PI * 2);
    ctx.stroke();
    const images = [];

    var imageCount = 0;
    console.log(game["flop"].length);

    game["flop"].forEach(src => {
        var f = new Image();
        f.src = src["link"];
        f.onload = () => {
            imageCount += 1;
            if (imageCount === game["flop"].length) {
                console.log("In image count");
                for (var j = 0; j < game["flop"].length; j++) {
                    console.log("J is: " + j);
                    ctx.drawImage(images[j], x2 / 2 + x1 - (card_width * 5) / 2 + (75 * j), y2 / 2 + y1 - card_height / 2, card_width, card_height);
                }
            }
        }
        images.push(f);

    });
    // console.log("I is: " + i);

    // f.onload = function () {
    //     console.log("In onload");
    //     //ctx.rotate();
    //     //ctx.drawImage(f, x2 / 2 + x1 - card_width * (5 - i) / 2, y2 / 2 + y1 - card_height / 2, card_width, card_height);
    //     if (imageCount == game["flop"].length - 1) {
    //         console.log("In image count");
    //         for (var j = 0; j < game["flop"].length; j++) {
    //             console.log("J is: " + j);
    //             ctx.drawImage(f, x2 / 2 + x1 - (card_width * 5) / 2 + (75 * j), y2 / 2 + y1 - card_height / 2, card_width, card_height);
    //         }
    //     }
    // }

    // f.src = game["flop"][i]["link"];
    // }
    // for (card of game["flop"]) {
    //     var f = new Image();
    //     f.onload = function () {
    //         //ctx.rotate();
    //         ctx.drawImage(f, x2 / 2 + x1 - card_width * 5 / 2, y2 / 2 + y1 - card_height / 2, card_width, card_height);
    //     }

    //     f.src = game["flop"][0]["link"];
    // }
    /*
    var flop_1 = new Image();
    flop_1.onload = function () {
        //ctx.rotate();
        ctx.drawImage(flop_1, x2 / 2 + x1 - card_width * 5 / 2, y2 / 2 + y1 - card_height / 2, card_width, card_height);
    }

    flop_1.src = game["flop"][0]["link"];

    var flop_2 = new Image();
    flop_2.onload = function () {
        //ctx.rotate();
        ctx.drawImage(flop_2, x2 / 2 + x1 - card_width * 3 / 2, y2 / 2 + y1 - card_height / 2, card_width, card_height);
    }

    flop_2.src = game["flop"][1]["link"];

    var flop_3 = new Image();
    flop_3.onload = function () {
        //ctx.rotate();
        ctx.drawImage(flop_3, x2 / 2 + x1 - card_width / 2, y2 / 2 + y1 - card_height / 2, card_width, card_height);
    }


    flop_3.src = game["flop"][2]["link"];
    */
}

function draw_winner(game, ctx) {
    const rect = canvas.getBoundingClientRect();
    ctx.font = '30px serif';
    ctx.fontStyle = "aqua";
    ctx.fillText(game["winner"]["ip"] + " wins with score " + game["winner"]["handvalue"], 50, 50);
}

//function drawCards() { }

var button1 = document.getElementById("player0");
button1.style = "position: absolute; top:300px; left:200px";
// button1.onclick = "myFunction()";
// var moves0 = document.getElementById("moves0");
// moves0.style = "position: absolute; top:100px; left:500px"


var canvas = document.getElementById('canvas');
const c_height = canvas.height;
const c_width = canvas.width;
var x1 = c_width * 0.15;
var y1 = c_height * 0.15;
var x2 = c_width * 0.7;
var y2 = c_height * 0.7;
var r = 125;
var ctx = canvas.getContext('2d');

var number_of_player = 6;
if (number_of_player > 6) {
    number_of_player = 6;
}


// var a = wasm.new_game();
// var window_height = $(window).height();
// var window_width = $(window).width();
// console.log(window_height);
// console.log(window_width);
var a = wasm.start_game_from_js(number_of_player);
console.log(a);
// a = wasm.flop_round_from_js(a);
// a = wasm.other_rounds_from_js(a);
// a = wasm.other_rounds_from_js(a);
// a = wasm.end_game(a);
// console.log(a);
// drawMainTable(ctx, x1, y1, x2, y2, r);
// draw_players(a, ctx, x1, y1, x2, y2, r);
// draw_names_and_scores(a, ctx, x1, y1, x2, y2, r);
// draw_flop(a, ctx, x1, y1, x2, y2, r);
m();
function m() {
    if (current_pos == 0) {
        drawMainTable(ctx, x1, y1, x2, y2, r);
        draw_players(a, ctx, x1, y1, x2, y2, r);
        draw_names_and_scores(a, ctx, x1, y1, x2, y2, r);
    } else if (current_pos == 1) {
        ctx.clearRect(0, 0, c_width, c_height);
        a = wasm.flop_round_from_js(a);
        drawMainTable(ctx, x1, y1, x2, y2, r);
        draw_players(a, ctx, x1, y1, x2, y2, r);
        draw_flop(a, ctx, x1, y1, x2, y2, r);
        draw_names_and_scores(a, ctx, x1, y1, x2, y2, r);
        redraw_button();
    } else if (current_pos == 2) {
        a = wasm.other_rounds_from_js(a);
        ctx.clearRect(0, 0, c_width, c_height);
        drawMainTable(ctx, x1, y1, x2, y2, r);
        draw_players(a, ctx, x1, y1, x2, y2, r);
        draw_flop(a, ctx, x1, y1, x2, y2, r);
        draw_names_and_scores(a, ctx, x1, y1, x2, y2, r);
        redraw_button();
    } else if (current_pos == 3) {
        a = wasm.other_rounds_from_js(a);
        ctx.clearRect(0, 0, c_width, c_height);
        drawMainTable(ctx, x1, y1, x2, y2, r);
        draw_players(a, ctx, x1, y1, x2, y2, r);
        draw_flop(a, ctx, x1, y1, x2, y2, r);
        draw_names_and_scores(a, ctx, x1, y1, x2, y2, r);
        redraw_button();
    } else {
        // Game is done
        a = wasm.end_game(a);
        ctx.clearRect(0, 0, c_width, c_height);
        drawMainTable(ctx, x1, y1, x2, y2, r);
        draw_players(a, ctx, x1, y1, x2, y2, r);
        draw_flop(a, ctx, x1, y1, x2, y2, r);
        draw_names_and_scores(a, ctx, x1, y1, x2, y2, r);
        draw_winner(a, ctx, x1, y1, x2, y2, r)
        console.log(a);
    }

    // a = wasm.other_rounds_from_js(a);

    // console.log(a);

    // a = wasm.other_rounds_from_js(a);

    // console.log(a);

    // var b = wasm.person_to_js();
    // console.log(b);
    // b = wasm.increment_num(b);

    // console.log(b);
    // b = wasm.basically_new_person(b);
    // console.log(b)

}
draw_next_button();
