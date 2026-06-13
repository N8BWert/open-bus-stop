"""

"""

import os
import argparse
import struct
from enum import Enum
import polars as pl
from dataclasses import dataclass


class SpecialServiceType(Enum):
    """
    Helper enum to identify the type of special service day for a given service id in the calendar_dates.txt file 
    """
    WEEKDAY = 0
    SATURDAY = 1
    SUNDAY = 2


@dataclass
class Date:
    """
    Helper class to parse and define the date format given in the GTFS data
    """
    year: int
    month: int
    day: int

    @classmethod
    def from_string(cls, date_str: str) -> "Date":
        """
        Parse a date string in the format YYYYMMDD and return a Date object. 
        """
        year = int(date_str[0:4])
        month = int(date_str[4:6])
        day = int(date_str[6:8])
        return cls(year, month, day)
    
    def ymd(self) -> tuple[int, int, int]:
        """
        Return the date as a tuple of (year, month, day)
        """
        return self.year, self.month, self.day


@dataclass
class Time:
    """
    Helper class to parse and define the time format given in the GTFS data 
    """
    hour: int
    minute: int
    second: int

    @classmethod
    def from_string(cls, time_str: str) -> "Time":
        """
        Parse a time string in the format HH:MM:SS and return a Time object. 
        """
        hour = int(time_str[0:2])
        minute = int(time_str[3:5])
        second = int(time_str[6:8])
        return cls(hour, minute, second)
    
    def hms(self) -> tuple[int, int, int]:
        """
        Return the time as a tuple of (hour, minute, second)
        """
        return self.hour, self.minute, self.second


@dataclass
class ServiceInfo:
    """
    Helper struct to organize the service information for a transit agency from the GTFS feed.
    """
    weekday_service_id: int
    saturday_service_id: int
    sunday_service_id: int
    special_service_ids: list[tuple[Date, SpecialServiceType, int]]


def get_stop_id() -> int:
    """
    Prompt the user for the id of the stop where the display will be located.

    Returns:
        int: The id of the stop where the display will be located.
    """
    stop_id = input("    1. What is the id of the stop this display will be located at? ")
    try:
        stop_id = int(stop_id)
    except ValueError:
        raise ValueError("Invalid stop id. Please enter a valid integer.")
    return stop_id

def get_route_ids(gtfs_path: str) -> tuple[list[int], list[int]]:
    """
    Prompt the user for the route numbers that serve the stop where the display will be located

    Args:
        gtfs_path (str): The path to the GTFS feed directory.
    Returns:
        tuple[list[int], list[int]]: A tuple containing two lists - the first is a list of route numbers that serve the stop, and the second is a list of corresponding route ids.
    """
    route_numbers_input = input("    2. What are the route numbers that serve this stop? (Please separate multiple route numbers with commas) ")
    route_numbers = [route.strip() for route in route_numbers_input.split(",")]
    routes_df = pl.read_csv(os.path.join(gtfs_path, "routes.txt"))
    route_ids = routes_df.filter(pl.col("route_short_name").is_in(route_numbers))["route_id"].to_list()
    return [int(route_number) for route_number in route_numbers], route_ids

def get_agency_service_info(gtfs_path: str) -> ServiceInfo:
    """
    Get the service information for the transit agency from the GTFS feed.

    Args:
        gtfs_path (str): The path to the GTFS feed directory.
    Returns:
        ServiceInfo: A dataclass containing the service information for the transit agency.
    """
    calendar_df = pl.read_csv(os.path.join(gtfs_path, "calendar.txt"))
    # NOTE: I'm not certain that all transit agencies will have the weekday and weekend service split MARTA has.
    # If this were to be fully rolled out, we would need to check that this is the case
    weekday_service_id = calendar_df.filter(pl.col("monday") == 1)["service_id"].to_list()[0]
    saturday_service_id = calendar_df.filter(pl.col("saturday") == 1)["service_id"].to_list()[0]
    sunday_service_id = calendar_df.filter(pl.col("sunday") == 1)["service_id"].to_list()[0]

    calendar_dates_df = pl.read_csv(os.path.join(gtfs_path, "calendar_dates.txt"))
    special_service_ids = []
    for chunk in calendar_dates_df.iter_slices(n_rows=2):
        special_service_id = chunk[0]["service_id"].item()
        date = Date.from_string(str(chunk[0]["date"].item()))
        new_service_id = chunk[1]["service_id"].item()
        if new_service_id == weekday_service_id:
            special_service_ids.append((date, SpecialServiceType.WEEKDAY, special_service_id))
        elif new_service_id == saturday_service_id:
            special_service_ids.append((date, SpecialServiceType.SATURDAY, special_service_id))
        elif new_service_id == sunday_service_id:
            special_service_ids.append((date, SpecialServiceType.SUNDAY, special_service_id))
    return ServiceInfo(weekday_service_id, saturday_service_id, sunday_service_id, special_service_ids)

def get_trips_for_route(gtfs_path: str, route_id: int, service_info: ServiceInfo) -> tuple[list[int], list[int], list[int]]:
    """
    Get the trip ids for the given route id for each of the three main service types (weekday, saturday, sunday).

    Args:
        gtfs_path (str): The path to the GTFS feed directory.
        route_id (int): The id of the route to get the trip ids for.
        service_info (ServiceInfo): A dataclass containing the service information for the transit agency.
    Returns:
        tuple[list[int], list[int], list[int]]: A tuple containing three lists of trip ids for the given route id for each of the three main service types (weekday, saturday, sunday).
    """
    trips_df = pl.read_csv(os.path.join(gtfs_path, "trips.txt"))
    route_trips = trips_df.filter(pl.col("route_id") == route_id)
    weekday_trips = route_trips.filter(pl.col("service_id") == service_info.weekday_service_id)["trip_id"].to_list()
    saturday_trips = route_trips.filter(pl.col("service_id") == service_info.saturday_service_id)["trip_id"].to_list()
    sunday_trips = route_trips.filter(pl.col("service_id") == service_info.sunday_service_id)["trip_id"].to_list()
    return weekday_trips, saturday_trips, sunday_trips

def get_stop_times_for_trip_ids(gtfs_path: str, trip_ids: list[int], stop_id: int) -> list[Time]:
    """
    Get the stop times for the given trip ids and stop id.

    Args:
        gtfs_path (str): The path to the GTFS feed directory.
        trip_ids (list[int]): A list of trip ids to get the stop times for.
        stop_id (int): The id of the stop to get the stop times for.

    Returns:
        list[Time]: A list of arrival times for the given trip ids and stop id.
    """
    stop_times_df = pl.read_csv(os.path.join(gtfs_path, "stop_times.txt"))
    trip_stop_times = stop_times_df.filter((pl.col("trip_id").is_in(trip_ids)) & (pl.col("stop_id") == stop_id))
    arrival_times = trip_stop_times["arrival_time"].to_list()
    return [Time.from_string(arrival_time) for arrival_time in arrival_times]

def pack_special_service_data(special_service_ids: list[tuple[Date, SpecialServiceType, int]]) -> bytes:
    """
    Pack the special service data into a byte array for use in the informal bus display firmware. Each special service day will be represented by 7 bytes in the following format:
        - 2 bytes for the year (unsigned short)
        - 1 byte for the month (unsigned char)
        - 1 byte for the day (unsigned char)
        - 1 byte for the special service type (unsigned char) 

    Args:
        special_service_ids (list[tuple[Date, SpecialServiceType, int]]): A list of tuples containing the date, special service type, and service id for each special service day.
    Returns:
        bytes: A byte array containing the packed special service data.
    """
    data = bytearray()
    for date, service_type, _ in special_service_ids:
        year, month, day = date.ymd()
        special_service_type_value = service_type.value
        data += struct.pack("<HBBB", year, month, day, special_service_type_value)
    return data

def pack_stop_times(stop_times: list[tuple[int, Time]]) -> bytes:
    """
    Pack the stop times into a byte array for use in the informal bus display firmware. Each stop time will be represented by 5 bytes in the following format:
        - 2 bytes for the route id (unsigned short)
        - 1 byte for the hour (unsigned char)
        - 1 byte for the minute (unsigned char)
        - 1 byte for the second (unsigned char)

    Args:
        stop_times (list[tuple[int, Time]]): A list of tuples containing the route id and stop time to be packed.
    Returns:
        bytes: A byte array containing the packed stop time data.
    """
    data = bytearray()
    for route_number, time in stop_times:
        hour, minute, second = time.hms()
        data += struct.pack("<HBBB", route_number, hour, minute, second)
    return data

def main(gtfs_path: str):
    print("Welcome to the Transit Feed Parser!")
    print("This tool helps to extract the relevant information from your transit agencies' GTFS feed for use in the informal bus displays.")
    print("")
    print("To get started, please answer the following questions:")
    stop_id = get_stop_id()
    route_numbers, route_ids = get_route_ids(gtfs_path)
    print("")
    print("...Parsing GTFS Feed...")
    agency_service_info = get_agency_service_info(gtfs_path)
    weekday_stop_times = []
    saturday_stop_times = []
    sunday_stop_times = []
    for (route_number, route_id) in zip(route_numbers, route_ids):
        weekday_trips, saturday_trips, sunday_trips = get_trips_for_route(gtfs_path, route_id, agency_service_info)
        weekday_stop_times += [(route_number, time) for time in get_stop_times_for_trip_ids(gtfs_path, weekday_trips, stop_id)]
        saturday_stop_times += [(route_number, time) for time in get_stop_times_for_trip_ids(gtfs_path, saturday_trips, stop_id)]
        sunday_stop_times += [(route_number, time) for time in get_stop_times_for_trip_ids(gtfs_path, sunday_trips, stop_id)]
    weekday_stop_times.sort(key=lambda x: x[1].hms())
    saturday_stop_times.sort(key=lambda x: x[1].hms())
    sunday_stop_times.sort(key=lambda x: x[1].hms())
    print("...Packing Data for Informal Bus Display Firmware...")
    packed_special_service_data = pack_special_service_data(agency_service_info.special_service_ids)
    packed_weekday_stop_times = pack_stop_times(weekday_stop_times)
    packed_saturday_stop_times = pack_stop_times(saturday_stop_times)
    packed_sunday_stop_times = pack_stop_times(sunday_stop_times)
    print("...Saving Packed Data to Files...")
    os.makedirs("../bus-stop-display/src/stop_data", exist_ok=True)
    with open("../bus-stop-display/src/stop_data/special_service_data.bin", "wb") as f:
        f.write(packed_special_service_data)
    with open("../bus-stop-display/src/stop_data/weekday_stop_times.bin", "wb") as f:
        f.write(packed_weekday_stop_times)
    with open("../bus-stop-display/src/stop_data/saturday_stop_times.bin", "wb") as f:
        f.write(packed_saturday_stop_times)
    with open("../bus-stop-display/src/stop_data/sunday_stop_times.bin", "wb") as f:
        f.write(packed_sunday_stop_times)
    print("Done! The packed data has been saved to the output directory.")


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description="Transit Feed Parser for Informal Bus Displays")
    parser.add_argument("gtfs_path", help="Path to the GTFS feed directory")
    args = parser.parse_args()
    main(args.gtfs_path)
