import * as wasm from "wasm-poker-game";

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
    ctx.lineTo(x2 + x1, y2 - r);
    // first two arguments is the bottom right corner of the rectangle and the next two is where the curve should end
    ctx.quadraticCurveTo(x2 + x1, y2 + y1, x2 + x1 - r, y1 + y2);
    ctx.lineTo(x1 + r, y2 + y1);
    ctx.quadraticCurveTo(x1, y2 + y1, x1, y1 + y2 - r);
    ctx.lineTo(x1, y1 + r)
    ctx.quadraticCurveTo(x1, y1, x1 + r, y1);
    ctx.fillStyle = "rgba(0,100,0,0.5)";
    ctx.fill();

}

function draw_cards(game, ctx, x1, y1, x2, y2, r) {
    ctx.beginPath();
    // Loc of Player 6 or the player of left side
    ctx.arc(x1, y2 / 2 + y1, 4, 0, Math.PI * 2);
    ctx.stroke();
    //ctx.moveTo(x2 + x1, y2 / 2 + y1);
    ctx.beginPath();
    // Loc of Player 3 or the right side
    ctx.arc(x2 + x1, y2 / 2 + y1, 4, 0, Math.PI * 2);
    ctx.stroke();
    // Top Two
    ctx.beginPath();
    // Loc of Player 1 or the topish left
    ctx.arc(x1 + x2 / 3, y1, 4, 0, Math.PI * 2);
    ctx.stroke();
    ctx.beginPath();
    // Loc of Player 2 or the topish right
    ctx.arc(x1 + x2 * 2 / 3, y1, 4, 0, Math.PI * 2);
    ctx.stroke();
    // Bottom two
    ctx.beginPath();
    // Loc of Player 1 or the topish left
    ctx.arc(x1 + x2 / 3, y1 + y2, 4, 0, Math.PI * 2);
    ctx.stroke();
    ctx.beginPath();
    // Loc of Player 2 or the topish right
    ctx.arc(x1 + x2 * 2 / 3, y1 + y2, 4, 0, Math.PI * 2);
    ctx.stroke();
}

//function drawCards() { }

var canvas = document.getElementById('canvas');
const c_height = canvas.height;
const c_width = canvas.width;
var x1 = c_width * 0.1;
var y1 = c_height * 0.1;
var x2 = c_width * 0.8;
var y2 = c_height * 0.8;
var r = 200;
var ctx = canvas.getContext('2d');
drawMainTable(ctx, x1, y1, x2, y2, r);
var number_of_player = 6;
// var a = wasm.new_game();

// console.log(a);

var a = wasm.start_game_from_js(number_of_player);
//a = wasm.add_player_from_js(a);
draw_cards(a, ctx, x1, y1, x2, y2, r);
console.log(a);

a = wasm.flop_round_from_js(a);

console.log(a);

a = wasm.other_rounds_from_js(a);

console.log(a);

a = wasm.other_rounds_from_js(a);

console.log(a);

// var b = wasm.person_to_js();
// console.log(b);
// b = wasm.increment_num(b);

// console.log(b);
// b = wasm.basically_new_person(b);
// console.log(b)

