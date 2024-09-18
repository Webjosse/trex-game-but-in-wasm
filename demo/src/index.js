import {init} from '../../pkg';

const CANVAS = document.getElementById("game");

const runner = init(CANVAS);

const rate = 1000 / 30; //30 fps -> ms



const game_loop =setInterval(() => {
    try{
        runner.update(rate);
    }catch(e){
        console.error(e);
        clearInterval(game_loop);
    }
}, rate)


document.body.addEventListener("keydown", evt => runner.send(evt))

document.body.addEventListener("keyup", evt => runner.send(evt))

document.body.addEventListener("keypress", evt => runner.send(evt))