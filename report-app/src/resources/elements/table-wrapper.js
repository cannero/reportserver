import {handsontable} from 'handsontable';
import {noView} from 'aurelia-framework';

//@noView(['handsontable/dist/handsontable.full.min.css'])
export class TableWrapper {
    hodata () {
        let keys = [];
        let table = handsontable;
        for(let key in table){
            keys.push(key);
        }
        return keys;
     }

}

