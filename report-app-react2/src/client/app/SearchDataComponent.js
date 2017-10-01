import React from 'react';
import ReactDOM from 'react-dom';
import HotTable from 'react-handsontable';
import xhr from 'xhr';
import {Button} from 'react-bootstrap';

class SearchDataComponent extends React.Component {
    state = {
        searchPart: '',
        message: 'not loaded',
        handsontableData: []
    }

    columnmapping = [
        {data: 'date'},
        {data: 'employee'},
        {data: 'customer'},
        {data: 'event'},
        {data: 'duration'},
        {data: 'comment'}];

    changeSearchPart = (evt) => {
        this.setState({
            searchPart: evt.target.value
        });
    };

    fetchData = (evt) => {
        evt.preventDefault();

        const url = 'http://localhost:3000/api/containingcomment?part=' +
                    encodeURIComponent(this.state.searchPart);
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
                const elements = json.result;

                self.setState({
                    handsontableData: elements,
                    message: "loaded " + elements.length
                });
            }
        });
    }

    render() {
        return (
            <div>
                <div>
                    <label>Search Term
                        <input placeholder="input" type="text"
                               value={this.state.searchPart} onChange={this.changeSearchPart}/>
                    </label>
                    <Button bsStyle="info" onClick={this.fetchData}>Search</Button>
                </div>
                <div id="hotSearchDataDiv">
                    <HotTable root="hotSearchData" columns={this.columnmapping} data={this.state.handsontableData} colHeaders={true} rowHeaders={true} height="750" stretchH="all" />
                </div>
                <div><span>{this.state.message}</span></div>
            </div>
        );
    }


}

export default SearchDataComponent;
