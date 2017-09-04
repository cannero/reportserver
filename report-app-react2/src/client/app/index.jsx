import React from 'react';
import {render} from 'react-dom';
import Sidebar from './Sidebar.jsx'
import {Grid, Row, Col} from 'react-bootstrap';
import AwesomeComponent from './AwesomeComponent.jsx';

class App extends React.Component {
    render() {
        return (
            <Grid fluid = {true}>
                <Row>
                    <Col md={2}>
                        <Sidebar />
                    </Col>
                    <Col md={10}>
                        <AwesomeComponent />
                    </Col>
                </Row>
            </Grid>
        );
    }
}

render(<App/>, document.getElementById('app'));
