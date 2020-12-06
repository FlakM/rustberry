create table sensors (
	id text primary key,
	name text
);



CREATE TABLE readings(
  time timestamptz primary key,
  sensor text references sensors(id),
  metric text,
  value real
  );
 
 
 create table water_history(
  	time timestamptz primary key,
    sensor text references sensors(id),
    duration_seconds real
 )



select * from readings r 
where sensor = '2'
order by sensor, time desc
;


ALTER table readings
ALTER COLUMN value TYPE real;


ALTER table water_history 
ALTER COLUMN time TYPE timestamp;

select * from readings r where time > now() - interval '60' day;


delete from readings ;

drop table sensors;
drop table readings ;