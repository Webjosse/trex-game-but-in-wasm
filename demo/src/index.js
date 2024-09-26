import {init} from '../../pkg';

const CANVAS = document.getElementById("game");

const runner = init(CANVAS, () => {
    const HTML_CLASSLIST = document.documentElement.classList;
    if (HTML_CLASSLIST.contains("night-mode")){
        HTML_CLASSLIST.remove("night-mode");
    } else {
        HTML_CLASSLIST.add("night-mode");
    }
});

function isChromiumBased() {
    const userAgent = navigator.userAgent.toLowerCase();
    return userAgent.includes('chrome') || userAgent.includes('chromium') || userAgent.includes('crios');
}
const rate = 1000 / (isChromiumBased() ? 60 : 30); //fps -> ms



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
CANVAS.addEventListener("click", evt => runner.send(evt))