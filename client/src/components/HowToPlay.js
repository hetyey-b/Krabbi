import React from "react";

import capture from "../images/howto/howto_capture.jpg";
import edgefort from "../images/howto/howto_edgefort.jpg";
import empty_board from "../images/howto/howto_empty_board.jpg";
import initial_board from "../images/howto/howto_initial_board.jpg";
import king_capture from "../images/howto/howto_king_capture.jpg";
import king_escape from "../images/howto/howto_king_escape.jpg";
import rook_movement from "../images/howto/howto_rook_movement.jpg";
import shieldwall from "../images/howto/howto_shieldwall.jpg";
import surround_win from "../images/howto/howto_surround_win.jpg";

const HowToPlay = () => {
    return (
        <div className="grid grid-cols-2 gap-2 items-center">
            <p>
                <span className="block font-bold underline">Basics</span>
                The two sides are the defenders/king's side (white) and the attackers (black).
                The attackers start with twice as many troops and they move first.
            </p>
            <img src={initial_board}/>

            <img src={rook_movement}/>
            <p>
                All pieces move any number of vacant squares along a row or column,
                like a rook in chess.
            </p>

            <p>
                The throne and the four corner squares are restricted. Only the king may 
                move on these squares.
            </p>
            <img src={empty_board}/>

            <img src={capture} />
            <p>
                <span className="block font-bold underline">Capturing</span>
                Pieces (except the king) are captured by being sandwiched between two enemies,
                or an enemy piece and a restricted square.
                Captures only happen on the aggressor's move, a piece moving in between two
                opponents is not captured.
            </p>

            <p>
                Groups of pieces may be captured along the board edge by fully surrounding them.
                This move is called a shieldwall capture.
            </p>
            <img src={shieldwall}/>

            <img src={king_escape}/>
            <p>
                <span className="block font-bold underline">Defender victory</span>
                The defenders win if the king escapes by reaching one of the restricted
                squares in the corners.
            </p>

            <p>
                The defenders also win if they can construct an escape fort.
                The escape fort is an unbreakable fort, where
                the king is able to move around, and is touching the edge of the board.
            </p>
            <img src={edgefort}/>

            <img src={king_capture}/>
            <p>
                <span className="block font-bold underline">Attacker victory</span>
                The attackers win by capturing the king. The king has to be captured
                by surrounding him on all four sides.
            </p>

            <p>
                The attackers also win by surrounding all remaining defender pieces
                in an unbreakable circle.
            </p>
            <img src={surround_win}/>

            <a 
                href="http://aagenielsen.dk/copenhagen_rules.php"
                className="underline col-span-2"
                target="_blank"
            >
                For even more detailed rules, check out the official Copenhagen Hnefatafl page
            </a>
        </div>
    )
};

export default HowToPlay;
