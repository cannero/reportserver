import React from 'react';
import ReactDOM from 'react-dom';
import HotTable from 'react-handsontable';
import xhr from 'xhr';
import {Button} from 'react-bootstrap';

class AwesomeComponent extends React.Component {

    constructor(props){
        super(props);
        this.state = {
            likesCount: 0,
            message: 'not loaded',
            handsontableData: []
        };
        this.columnmapping = [
            {data: 'date'},
            {data: 'employee'},
            {data: 'customer'},
            {data: 'event'},
            {data: 'duration'},
            {data: 'comment'}];
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
            if( err !== null ){
                console.log(err);
                self.setState({
                    message: "loading failed"
                });
            } else {
                const json = JSON.parse(data.body);
                self.setState({
                    handsontableData: json.result,
                    message: json.result.length
                });
            }
        });
    }

    render() {
        return (
            <div>
                Likes : <span>{this.state.likesCount}</span>
                <div><Button bsStyle="info" onClick={this.onLike}>Like Me</Button></div>
                <div id="example-component">
                    <HotTable root="hot" columns= {this.columnmapping} data={this.state.handsontableData} colHeaders={true} rowHeaders={true} width="1000" height="300" stretchH="all" />
                </div>
                <div><span>{this.state.message}</span></div>
            </div>
        );
    }
}

export default AwesomeComponent;

