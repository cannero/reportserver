import React from 'react';
import ReactDOM from 'react-dom';
import HotTable from 'react-handsontable';

class AwesomeComponent extends React.Component {

    constructor(props){
        super(props);
        this.state = {likesCount: 0},
        this.onLike = this.onLike.bind(this);
        this.handsontableData = [
            ["", "Ford", "Volvo", "Trabi"],
            ["2016", 10, 11, 12],
            ["2017", 10, 11, 500]
        ];
    }

    onLike(){
        let newLikesCount = this.state.likesCount + 1;
        this.setState({likesCount: newLikesCount});
    }

    render() {
        return (
            <div>
                Likes : <span>{this.state.likesCount}</span>
                <div><button onClick={this.onLike}>Like Me</button></div>
                <div id="example-component">
                    <HotTable root="hot" data={this.handsontableData} colHeaders={true} rowHeaders={true} width="600" height="300" stretchH="all" />
                </div>
            </div>
        );
    }
}

export default AwesomeComponent;

