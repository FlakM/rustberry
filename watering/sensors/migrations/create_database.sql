create table sensors (
	id text primary key,
	name text
);



CREATE TABLE readings(
  time timestamp primary key,
  sensor text references sensors(id),
  metric text,
  value real
  );
 
 
 select * from readings
 
 create table water_history(
  	time timestamp primary key,
    sensor text references sensors(id),
    duration_seconds real
 )

 

select * from readings r join sensors s on s.id  = r.sensor;

select * from water_history h join sensors s on s.id  = h.sensor;

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


INSERT INTO public.sensors (id,"name") VALUES ('1','bazylia i pietruszka'), ('2','mięta');



CREATE USER watering WITH PASSWORD 'Password1';