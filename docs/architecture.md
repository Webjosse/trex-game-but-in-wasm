# Code architecture

## Modules
| Name     | Role                                                       |
|----------|------------------------------------------------------------|
| Utils    | Global Rust utils, that could be used for other programs   |
| Engine   | A game execution logic, that could be used for other games |
| Events   | The events management for this game                        |
| Gameplay | The game logic, where the entrypoints are                  |

## Entrypoints
```ts
interface GameRunner{
  /**
   * Updates the canvas
   * @param deltaMs the time elapsed in milliseconds
   */
  update(deltaMs: number);

  /**
   * Sends an HTML event
   * @param evt the HTML event
   */
  send(evt:Event);
}
const init = (canvas: HtmlCanvasElement) => GameRunner;
```

For now, we:
* run on 30fps, so `deltaMs = 1000/30` ans update is called each `deltaMs` milliseconds
* send `KeyUp`, `KeyDown` and `KeyPress` events

## Engine
There is a hierarchy
* ~ engine ~ -> The entrypoint
  * controllers -> Does not change during the game, implementation is provided by the engine module
    * entities -> The game objects, to implement as you like
      * sprites(mandatory) -> A texture with a position
        * texture -> A drawable

## Entities
| Name          | Role                                  |
|---------------|---------------------------------------|
| BgEntity      | The background, with the floor        |
| CloudsSpawner | Used to spawn the clouds              |
| DinoEntity    | The dino                              |
| SpeedEntity   | Used only to increase speed over time |
