import * as moment from 'moment';

export const dateFilters = [
    { label: 'Last 1 Week', type: 'days', interval: 7 },
    { label: 'Last 2 Weeks', type: 'days', interval: 14 },
    { label: 'Last 1 Month', type: 'months', interval: 1 },
    { label: 'Last 3 Months', type: 'months', interval: 3 },
    { label: 'Last 6 Months', type: 'months', interval: 6 },
    { label: 'Last 1 Year', type: 'years', interval: 1 }
];

export function getDateRange(filter: any) {
    switch (filter.type) {
        case 'days':
            return getRange('days', filter.interval);
        case 'months':
            return getRange('months', filter.interval);
        case 'years':
            return getRange('years', filter.interval);
        default:
            // Should not happen this
            return getRange('days', 7);
    }
}

function getRange(type, interval) {
    const today = new Date();
    // toDate is exclusive, always one day forward
    const tomorrow = new Date(today);
    tomorrow.setDate(tomorrow.getDate() + 1);

    const from_date = moment(today).subtract(interval, type).format('YYYY-MM-DD');
    const to_date = moment(tomorrow).format('YYYY-MM-DD');

    return {
        fromDate: from_date,
        toDate: to_date
    };
}
