import ReactDOM from "react-dom/client"
import Chessground from "@react-chess/chessground"

import "chessground/assets/chessground.base.css"
import "chessground/assets/chessground.brown.css"
import "chessground/assets/chessground.cburnett.css"

function App() {
  return (
    <div>
      <Chessground
        width={500}
        height={500}
        config={{
          // fen: "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 3",
          events: {
            move: (orig: string, dest: string) => {
              console.log(orig, dest)
            },
          },
          movable: {
            showDests: true,
            dests: new Map([["e2", ["e4", "e3"]]]),
          },
        }}
      />
    </div>
  )
}

ReactDOM.createRoot(document.getElementById("app")!).render(<App />)
