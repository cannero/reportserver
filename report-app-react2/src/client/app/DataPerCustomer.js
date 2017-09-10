import React from 'react';
import ReactDOM from 'react-dom';
import xhr from 'xhr';
import Plot from './Plot.jsx';

class DataPerCustomer extends React.Component {

    state = {
        message: '',
        numDays: 5,
        customers: [],
        durations: [],
        totalDuration: 0
    }

    changeNumDays = (evt) => {
        this.setState({
            numDays: evt.target.value
        });
    };

    fetchData = (evt) => {
        evt.preventDefault();

        const url = 'http://localhost:3000/api/timepercustomer';
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
                let customers = [];
                let durations = [];
                for (var i = 0; i < elements.length; i++){
                    const elem = elements[i];
                    customers.push(elem.customer);
                    durations.push(elem.duration);
                }
                
                self.setState({
                    customers: customers,
                    durations: durations,
                    totalDuration: json.total_duration,
                    message: json.result.length
                });
            }
        });
    }

    render() {
        return (
            <div>
                <form onSubmit={this.fetchData}>
                    <label>Days
                        <input placeholder={"5"} type="text"
                        value={this.state.numDays} onChange={this.changeNumDays}/>
                    </label>
                </form>
                {(this.state.customers.length > 0) ? (
                    <Plot xData = {this.state.customers}
                          yData = {this.state.durations}
                          type = "bar"
                    />) : null}
                <div><span>{this.state.totalDuration}</span></div>
                <div><span>{this.state.message}</span></div>
            </div>
        );
    }
}

export default DataPerCustomer;
