import React, { Component } from 'react';
import './App.css';

class ReportSelectForm extends Component {
    constructor(props){
        super(props);
        this.state = {value: 'report1'};

        this.handleChange = this.handleChange.bind(this);
        this.handleSubmit = this.handleSubmit.bind(this);
    }

    handleChange(event){
        this.setState({value: event.target.value});
    }

    handleSubmit(event){
        this.props.onSelected(this.state.value);
        event.preventDefault();
    }

    render() {
        return (
                <form onSubmit={this.handleSubmit}>
                <label>
                Report:
                <select value={this.state.value}
                        onChange={this.handleChange}>
                <option value="report1">All</option>
                <option value="report2">Last week</option>
                <option value="report3">Project specific</option>
                </select>
                </label>
                <input type="submit" value="Submit" />
                </form>
        );
    }
}

class App extends Component {

    handleSelection(report){
        alert('selected report: ' + report);
    }
    
    render() {
        return (
                <div className="App">
                <div className="App-header">
                <h2>Welcome to Report</h2>
                </div>
                <ReportSelectForm onSelected={(report) => this.handleSelection(report)}/>
                </div>
        );
    }
}

export default App;
