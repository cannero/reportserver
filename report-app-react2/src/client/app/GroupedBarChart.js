/* global Plotly */
import React from 'react';

class GroupedBarChart extends React.Component {

    drawPlot = () => {
        Plotly.newPlot('groupedPlot', [
            this.props.trace1, this.props.trace2
        ], {
            margin: {
                t: 0, r: 0, l: 30
            }
        }, {
            barmode: 'group'
        });
    }
            
    componentDidMount() {
        this.drawPlot();
    }

    componentDidUpdate() {
        this.drawPlot();
    }
    
    render(){
        return (
            <div id="groupedPlot">
            </div>
        );
    }
}

export default GroupedBarChart;
