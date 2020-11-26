create table sensors (
	id text primary key,
	name text
);



CREATE TABLE readings(
  time timestamptz primary key,
  sensor text references sensors(id),
  metric text,
  value integer
  );
 
 
 create table water_history(
  	time timestamptz primary key,
    sensor text references sensors(id),
    duration_seconds integer
 )
 



 
insert into sensors values ('1', 'bazylia i pietruszka');
insert into sensors values ('2', 'mięta');
 

insert into readings (time, sensor, metric, value) values ( now(), '1', 'humidity', 80.0 );


select * from readings
where time > 


select * from readings r 
where sensor = '2'
order by sensor, time desc
;


ALTER table readings
ALTER COLUMN value TYPE real;


ALTER table readings
ALTER COLUMN time TYPE timestamp;

select * from readings r where time > now() - interval '60' day;


delete from readings ;

drop table sensors;
drop table readings ;