import React from 'react';
import {render} from 'react-dom';
import Sidebar from './Sidebar.jsx'
import {Grid, Row, Col} from 'react-bootstrap';
import AwesomeComponent from './AwesomeComponent.jsx';
import DataPerCustomer from './DataPerCustomer.js';
import SearchDataComponent from './SearchDataComponent.js';

class App extends React.Component {
    render() {
        return (
            <Grid fluid = {true}>
                <Row>
                    <Col md={2} sm={2} className="sidebarcolumn">
                        <Sidebar />
                    </Col>
                    <Col md={10} sm={10}>
                    <Row>
                        <SearchDataComponent />
                    </Row>
                    <Row>
                        <AwesomeComponent />
                    </Row>
                    <Row>
                        <DataPerCustomer />
                    </Row>
                    </Col>
                </Row>
            </Grid>
        );
    }
}

render(<App/>, document.getElementById('app'));
