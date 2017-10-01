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
            handsontableData: [],
            numDays: 5
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

    changeNumDays = (evt) => {
        this.setState({
            numDays: evt.target.value
        });
    };

    onLike(){
        let newLikesCount = this.state.likesCount + 1;
        this.setState({likesCount: newLikesCount});
        const days = this.state.numDays;
        let url = 'http://localhost:3000/api/foremployee';
        if(days > 0){
            url = url + '?days=' + encodeURIComponent(days);
        }

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
                <label>Days
                    <input placeholder={this.state.numDays} type="text"
                           value={this.state.numDays} onChange={this.changeNumDays}/>
                </label>
                <div><Button bsStyle="info" onClick={this.onLike}>Like Me</Button></div>
                <div id="example-component">
                    <HotTable root="hot" columns= {this.columnmapping} data={this.state.handsontableData} colHeaders={true} rowHeaders={true} height="750" stretchH="all" />
                </div>
                <div><span>{this.state.message}</span></div>
            </div>
        );
    }
}

export default AwesomeComponent;

