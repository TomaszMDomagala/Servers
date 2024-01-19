CREATE TABLE servers (
  server_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL UNIQUE,
  server_name VARCHAR(30) NOT NULL,
  server_address VARCHAR(20) NOT NULL,
  server_username VARCHAR(30) NOT NULL,
  server_password VARCHAR(50) NOT NULL,
  server_available BOOLEAN
);

CREATE TABLE services (
  service_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL UNIQUE,
  service_name VARCHAR(50) NOT NULL,
  service_port INT NOT NULL,
  service_username VARCHAR(30),
  service_password VARCHAR(50),
  service_available BOOLEAN,
  server_id INT,
  FOREIGN KEY (server_id) REFERENCES servers(server_id)
);

CREATE TABLE containers (
  container_id INT AUTO_INCREMENT PRIMARY KEY NOT NULL UNIQUE,
  container_name VARCHAR(150),
  container_image VARCHAR(150),
  container_state VARCHAR(100),
  container_status VARCHAR(100),
  service_id INT,
  FOREIGN KEY (service_id) REFERENCES services(service_id)
);
