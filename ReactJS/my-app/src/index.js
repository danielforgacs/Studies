import React from 'react';
import ReactDOM from 'react-dom';
import './index.css';



class Square extends React.Component {
    render () {
        return (
            <button className="square" onClick={() => console.log('LKHJ')}>
                {this.props.value}
            </button>
        );
    }
}

class Board extends React.Component {
    constructor (props) {
        super (props)
        this.state = {
            squares: Array(9).fill(null)
        }
    }

    handleClick (i) {
        console.log("CLICKED")

    }

    renderSquare(i) {
        return (
            <Square value={this.state.squares[i]} onClick={ () => this.handleClick(i) } />
        )
    }

    render () {
        console.log(this.state)

        return (
            <div>
                <div className="board-row">
                    {this.renderSquare(0)}
                    {this.renderSquare(1)}
                    {this.renderSquare(2)}
                </div>
                <div className="board-row">
                    {this.renderSquare(3)}
                    {this.renderSquare(4)}
                    {this.renderSquare(5)}
                </div>
                <div className="board-row">
                    {this.renderSquare(6)}
                    {this.renderSquare(7)}
                    {this.renderSquare(8)}
                </div>
            </div>
        );
    }
}

class Game extends React.Component {
    render() {
        return (
            <div className="game">
                <div className="game-board">
                    <Board />
                </div>
                <div className="game-info">
                </div>
            </div>
        );
    }
}

class Extra extends React.Component {
    render () {
        return (
            <div>
                <Game />
            </div>
        )
    }
}

ReactDOM.render(
    <Extra />,
    document.getElementById('root')
);
