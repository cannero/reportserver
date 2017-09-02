import React from 'react';
import ReactDOM from 'react-dom';
import HotTable from 'react-handsontable';
import xhr from 'xhr';

class AwesomeComponent extends React.Component {

    constructor(props){
        super(props);
        this.state = {
            likesCount: 0,
            message: 'not loaded',
            handsontableData: [
            ["", "Ford", "Volvo", "Trabi"],
            ["2016", 10, 11, 12],
            ["2017", 10, 11, 500]]
        },
        this.onLike = this.onLike.bind(this);
    }

    onLike(){
        let newLikesCount = this.state.likesCount + 1;
        this.setState({likesCount: newLikesCount});

        const url = 'http://localhost:3000/api/foremployee';
        const self = this;
        xhr({
            url: url
        }, function(err, data){
            console.log(err);
            const json = JSON.parse(data.body);
            self.setState({
                handsontableData: json.result,
                message: json.result.length
            });
        });
    }

    render() {
        return (
            <div>
                Likes : <span>{this.state.likesCount}</span>
                <div><button onClick={this.onLike}>Like Me</button></div>
                <div id="example-component">
                    <HotTable root="hot" data={this.state.handsontableData} colHeaders={true} rowHeaders={true} width="1000" height="300" stretchH="all" />
                </div>
                <div><span>{this.state.message}</span></div>
            </div>
        );
    }
}

export default AwesomeComponent;

