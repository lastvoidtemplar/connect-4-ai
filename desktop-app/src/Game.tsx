import { invoke } from "@tauri-apps/api/core";
import React from "react";

const WIDTH = 7;
const HEIGHT = 6;

function buildBoard(encodedBoard: string): number[][] {
  const board = Array.from({ length: HEIGHT }, () => Array(WIDTH).fill(0));

  for (let i = 0; i < encodedBoard.length; i++) {
    const col = parseInt(encodedBoard[i]) - 1;
    for (let row = HEIGHT - 1; row >= 0; row--) {
      if (board[row][col] === 0) {
        board[row][col] = i % 2 === 0 ? 1 : 2;
        break;
      }
    }
  }

  return board;
}

function Game() {
  const [scores, setScores] = React.useState<(number | null)[]>(
    Array(WIDTH).fill(null),
  );
  const [encodedBoard, setEncodedBoard] = React.useState("");

  React.useEffect(() => {
    refresh();
  }, []);

  async function refresh() {
    try {
      const scores = await invoke<(number | null)[]>("columns_score");
      const encodedBoard = await invoke<string>("get_encoded_board");
      setScores(scores);
      setEncodedBoard(encodedBoard);
      console.log(scores);
    } catch (err) {
      console.error(err);
    }
  }

  const board = buildBoard(encodedBoard);

  async function playColm(colm: number) {
    try {
      await invoke("play_colm", { colm });
      await refresh();
    } catch (err) {
      alert(err);
    }
  }

  async function backMove() {
    await invoke("back_move");
    await refresh();
  }

  async function resetGame() {
    await invoke("reset_game");
    await refresh();
  }

  return (
    <div className="flex flex-col items-center min-h-screen bg-gray-900 text-white py-20">
      <div className="grid grid-cols-7 gap-2 bg-blue-800 p-4 rounded-2xl">
        {board.map((row, r) =>
          row.map((cell, c) => (
            <div
              key={`${r}-${c}`}
              className="w-24 h-24 bg-white rounded-full flex items-center justify-center cursor-pointer hover:scale-105 transition"
              onClick={() => playColm(c + 1)}
            >
              {cell === 1 && (
                <div className="w-20 h-20 bg-red-500 rounded-full" />
              )}
              {cell === 2 && (
                <div className="w-20 h-20 bg-yellow-400 rounded-full" />
              )}
            </div>
          )),
        )}
      </div>

      <div className="grid grid-cols-7 gap-2 mt-3 text-center">
        {scores.map((score, i) => (
          <div key={i} className="text-2xl font-semibold w-24">
            {score !== null ? score : "-"}
          </div>
        ))}
      </div>

      <div className="flex gap-2">
        <button
          onClick={backMove}
          className="mt-8 px-4 py-2 bg-gray-700 rounded-lg hover:bg-gray-600 transition"
        >
          Back
        </button>
        <button
          onClick={resetGame}
          className="mt-8 px-4 py-2 bg-gray-700 rounded-lg hover:bg-gray-600 transition"
        >
          Reset
        </button>
      </div>
    </div>
  );
}

export default Game;
