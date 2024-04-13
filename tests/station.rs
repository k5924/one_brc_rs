use one_brc_rs::station::Station;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_station_new() {
        let station = Station::new(10.0);
        assert_eq!(station.min, 10.0);
        assert_eq!(station.max, 10.0);
        assert_eq!(station.sum, 10.0);
        assert_eq!(station.count, 1);
    }

    #[test]
    fn test_station_update() {
        let mut station = Station::new(10.0);
        station.update(20.0);
        assert_eq!(station.min, 10.0);
        assert_eq!(station.max, 20.0);
        assert_eq!(station.sum, 30.0);
        assert_eq!(station.count, 2);
    }

    #[test]
    fn test_station_merge() {
        let mut station1 = Station::new(10.0);
        let station2 = Station::new(20.0);
        station1.merge(station2);
        assert_eq!(station1.min, 10.0);
        assert_eq!(station1.max, 20.0);
        assert_eq!(station1.sum, 30.0);
        assert_eq!(station1.count, 2);
    }

    #[test]
    fn test_station_display() {
        let station = Station {
            min: 10.0,
            max: 20.0,
            sum: 30.0,
            count: 2,
        };
        assert_eq!(format!("{}", station), "10.0/20.0/15.0");
    }
}
