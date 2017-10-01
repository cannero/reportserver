import React from 'react';
import ReactDOM from 'react-dom';
import xhr from 'xhr';
import Plot from './Plot.jsx';
import GroupedBarChart from './GroupedBarChart.js';

class DataPerCustomer extends React.Component {

    state = {
        message: '',
        numDays: 5,
        customers: [],
        durations: [],
        perDivision: {
            arad: {
                customers: [],
                durations: []
            },
            krailling: {
                customers: [],
                durations: []
            }
        },
        totalDuration: 0
    }

    changeNumDays = (evt) => {
        this.setState({
            numDays: evt.target.value
        });
    };

    fetchData = (evt) => {
        evt.preventDefault();

        const days = this.state.numDays;
        let url = 'http://localhost:3000/api/timepercustomer';
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

        let url_per_division = 'http://localhost:3000/api/timepercustomerdivision';
        if(days > 0){
            url_per_division = url_per_division + '?days=' + encodeURIComponent(days);
        }
        
        xhr({
            url: url_per_division
        }, function(err, data){
            if( err !== null ){
                console.log(err);
                self.setState({
                    message: "loading failed division"
                });
            } else {
                const json = JSON.parse(data.body);
                const elements = json.result;
                let customers_arad = [];
                let durations_arad = [];
                let customers_krailling = [];
                let durations_krailling = [];

                for (var i = 0; i < elements.length; i++){
                    const elem = elements[i];
                    if(elem.division === "Arad"){
                        customers_arad.push(elem.customer);
                        durations_arad.push(elem.duration);
                    } else {
                        customers_krailling.push(elem.customer);
                        durations_krailling.push(elem.duration);
                    }
                }
                
                self.setState({
                    perDivision:{
                        arad: {
                            customers: customers_arad,
                            durations: durations_arad
                        },
                        krailling: {
                            customers: customers_krailling,
                            durations: durations_krailling
                        }
                    },
                    message: "per div" + json.result.length
                });
            }
        });

    }

    render() {
        const trace1 = {x: this.state.perDivision.arad.customers,
                        y: this.state.perDivision.arad.durations,
                        name: 'arad',
                        type: 'bar'};
        const trace2 = {x: this.state.perDivision.krailling.customers,
                        y: this.state.perDivision.krailling.durations,
                        name: 'krailling',
                        type: 'bar'}
        
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
                {(trace1.x.length > 0 || trace2.x.length >0) ? (
                <GroupedBarChart trace1 = {trace1}
                                 trace2 = {trace2} /> ) : null }
            </div>
        );
    }
}

export default DataPerCustomer;
