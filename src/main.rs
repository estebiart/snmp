/**use snmp::{SyncSession, Value};
use std::time::Duration;

fn main() {
    let target_ip = "192.168.1.1:161"; // Cambia a la IP de tu impresora
    let community = "public"; // Comunidad SNMP
    let oid: &[u32] = &[1, 3, 6, 1, 2, 1, 1, 5, 0]; // OID para nombre del dispositivo

    // Establecer sesión SNMP
    match SyncSession::new(
        target_ip,
        community.as_bytes(),
        Some(Duration::from_secs(2)),
        0, // Request ID inicial
    ) {
        Ok(mut session) => {
            // Obtener datos SNMP
            match session.get(oid) {
                Ok(response) => {
                    // Corrige el acceso a varbinds
                    if let Some(varbind) = response.varbinds.into_iter().next() {
                        if let Value::OctetString(value) = varbind.value {
                            println!("Nombre del dispositivo: {}", String::from_utf8_lossy(&value));
                        } else {
                            println!("El OID no devolvió un valor tipo OctetString.");
                        }
                    } else {
                        println!("No se encontró ningún varbind en la respuesta.");
                    }
                }
                Err(e) => {
                    println!("Error al obtener SNMP: {:?}", e); // Usar {:?} para mostrar errores
                }
            }
        }
        Err(e) => {
            println!("Error al establecer sesión SNMP: {:?}", e);
        }
    }
}
**/

// use snmp::{SyncSession, Value};
// // use ipnetwork::IpNetwork;
// // use std::time::Duration;
// use std::process::Command;

// fn main() {
//     let rustscan_output = Command::new("rustscan")
//         .args(["-a", "192.168.1.0/24", "--ulimit", "5000", "-p", "161"])
//         .output()
//         .expect("Failed to execute RustScan");

//     let output = String::from_utf8_lossy(&rustscan_output.stdout);
//     println!("RustScan Output: {}", output);

//     // Parse the RustScan output to extract IPs with port 161 open
//     let ips: Vec<&str> = output
//         .lines()
//         .filter(|line| line.contains("Open"))
//         .map(|line| line.split_whitespace().next().unwrap_or(""))
//         .collect();

//     println!("Devices with port 161 open: {:?}", ips);
// }

// fn main() {
//     let network = "192.168.1.0/24".parse::<IpNetwork>().expect("Invalid network format");
//     let common_oid = &[1, 3, 6, 1, 2, 1, 1, 5, 0]; // OID for System Name (device name)
//     let community = "public"; // Default SNMP community string

//     println!("Scanning network: {}", network);

//     for ip in network.iter() {
//         match SyncSession::new(&format!("{}:161", ip), community.as_bytes(), Some(Duration::from_secs(1)), 0) {
//             Ok(mut session) => {
//                 match session.get(common_oid) {
//                     Ok(response) => {
//                         if let Some((_, value)) = response.varbinds.into_iter().next() {
//                             match value {
//                                 Value::OctetString(value) => {
//                                     println!(
//                                         "Device found at {}: Name: {}",
//                                         ip,
//                                         String::from_utf8_lossy(&value)
//                                     );
//                                 }
//                                 _ => println!("Device at {} returned an unexpected value.", ip),
//                             }
//                         } else {
//                             println!("Device at {} did not return any varbinds.", ip);
//                         }
//                     }
//                     Err(_) => {
//                         println!("No SNMP response from {}", ip);
//                     }
//                 }
//             }
//             Err(_) => {
//                 println!("Failed to establish SNMP session with {}", ip);
//             }
//         }
//     }
// }

// fn main() {
//     inmutable()
//     let x: i32 = 5;
// }
// fn inmutable() {
//     let mut x = 5;
//     println!("The value of x is: {}", x);
//     x = 6;
//     println!("The value of x is: {}", x);
// }

// use ipnetwork::IpNetwork;
// use snmp::{SyncSession, Value};
// use std::fmt::Error;
// use std::process::Command;
// use std::time::Duration;

// fn main() {
//     // Configuración
//     let network = "192.168.1.0/24".parse::<IpNetwork>().expect("Formato de red inválido");
//     let community = "public"; // Cambia si tu red usa otra comunidad SNMP
//     let timeout = Duration::from_secs(2);

//     println!("Escaneando la red para dispositivos SNMP...");

//     // Paso 1: Detectar dispositivos con RustScan
//     let rustscan_output = Command::new("rustscan")
//         .args(["-a", &network.to_string(), "--ulimit", "5000", "-p", "161"])
//         .output()
//         .expect("No se pudo ejecutar RustScan");

//     let output = String::from_utf8_lossy(&rustscan_output.stdout);
//     let ips: Vec<&str> = output
//         .lines()
//         .filter(|line| line.contains("Open")) // Filtra líneas con puertos abiertos
//         .map(|line| line.split_whitespace().next().unwrap_or(""))
//         .collect();

//     println!("IPs con puerto 161 abierto: {:?}", ips);

//     // Paso 2: Consultar dispositivos SNMP
//     for ip in ips {
//         println!("Conectando a dispositivo en {}...", ip);

//         match SyncSession::new(&format!("{}:161", ip), community.as_bytes(), Some(timeout), 0) {
//             Ok(mut session) => {
//                 // Verificar si es una impresora
//                 if let Ok(description) = get_oid_string(&mut session, &[1, 3, 6, 1, 2, 1, 1, 1, 0]) {
//                     println!("Descripción del sistema en {}: {}", ip, description);

//                     if description.contains("Printer") || description.contains("HP") || description.contains("Canon") {
//                         println!("Dispositivo identificado como impresora en {}.", ip);

//                         // Extraer información adicional
//                         if let Ok(serial_number) = get_oid_string(&mut session, &[1, 3, 6, 1, 2, 1, 43, 5, 1, 1, 16, 1]) {
//                             println!("Número de serie: {}", serial_number);
//                         }
//                         if let Ok(status) = get_oid_string(&mut session, &[1, 3, 6, 1, 2, 1, 43, 10, 2, 1, 4, 1, 1]) {
//                             println!("Estado de la impresora: {}", status);
//                         }
//                     } else {
//                         println!("Dispositivo en {} no parece ser una impresora.", ip);
//                     }
//                 } else {
//                     println!("No se pudo obtener la descripción del dispositivo en {}.", ip);
//                 }
//             }
//             Err(_) => println!("No se pudo establecer conexión SNMP con {}.", ip),
//         }
//     }
// }

// /// Función para obtener un OID como string
// fn get_oid_string(session: &mut SyncSession, oid: &[u32]) -> Result<String,Error> {
//     let response = session.get(oid);
//     if let Some((_, Value::OctetString(value))) = response.unwrap().varbinds.into_iter().next() {
//         return Ok(String::from_utf8_lossy(&value).to_string());
//     }
//     Err(Error)
// }
// use ipnetwork::IpNetwork;
// use snmp::{SyncSession, Value};
// use std::fmt::Error;
// use std::process::Command;
// use std::time::Duration;

// fn main() {
//     // Configuración inicial
//     let network = "192.168.1.0/24".parse::<IpNetwork>().expect("Formato de red inválido");
//     let community = "public"; // Cambia si tu red usa otra comunidad SNMP
//     let timeout = Duration::from_secs(2);

//     println!("Escaneando la red para dispositivos y puertos abiertos...");

//     // Paso 1: Detectar dispositivos y puertos abiertos con RustScan
//     let rustscan_output = Command::new("rustscan")
//         .args(["-a", &network.to_string(), "--ulimit", "5000", "-r", "1-6559"])
//         .output()
//         .expect("No se pudo ejecutar RustScan");

//     let output = String::from_utf8_lossy(&rustscan_output.stdout);
//     let devices: Vec<(String, Vec<u16>)> = parse_rustscan_output(&output);

//     println!("Dispositivos detectados con puertos abiertos: {:?}", devices);

//     // Paso 2: Consultar SNMP en cada puerto detectado
//     for (ip, ports) in devices {
//         println!("\nAnalizando dispositivo en {}...", ip);

//         for port in ports {
//             println!("  Probando puerto {}...", port);

//             match SyncSession::new(&format!("{}:{}", ip, port), community.as_bytes(), Some(timeout), 0) {
//                 Ok(mut session) => {
//                     // Consultar información básica
//                     if let Ok(description) = get_oid_string(&mut session, &[1, 3, 6, 1, 2, 1, 1, 1, 0]) {
//                         println!("    Descripción del sistema: {}", description);
//                     } else {
//                         println!("    No se pudo obtener descripción en el puerto {}.", port);
//                     }
//                 }
//                 Err(_) => {
//                     println!("    No se pudo establecer conexión SNMP en el puerto {}.", port);
//                 }
//             }
//         }
//     }
// }

// /// Función para analizar la salida de RustScan y extraer IPs y puertos abiertos
// fn parse_rustscan_output(output: &str) -> Vec<(String, Vec<u16>)> {
//     let mut devices = Vec::new();
//     let mut current_ip = String::new();
//     let mut current_ports = Vec::new();

//     for line in output.lines() {
//         if line.contains("Open") {
//             if !current_ip.is_empty() {
//                 devices.push((current_ip.clone(), current_ports.clone()));
//                 current_ports.clear();
//             }
//             current_ip = line.split_whitespace().next().unwrap_or("").to_string();
//         } else if let Ok(port) = line.trim().parse::<u16>() {
//             current_ports.push(port);
//         }
//     }

//     if !current_ip.is_empty() {
//         devices.push((current_ip, current_ports));
//     }

//     devices
// }

// /// Función para obtener un OID como string
// fn get_oid_string(session: &mut SyncSession, oid: &[u32]) -> Result<String, Error> {
//     let response = session.get(oid);
//     if let Some((_, Value::OctetString(value))) = response.unwrap().varbinds.into_iter().next() {
//         return Ok(String::from_utf8_lossy(&value).to_string());
//     }
//     Err(Error)
// }


// use ipnetwork::IpNetwork;
// use snmp::{SyncSession, Value};
// use std::process::Command;
// use std::time::Duration;
// use std::error::Error;

// fn main() {
//     // Configuración inicial
//     let network = "168.176.22.0/24".parse::<IpNetwork>().expect("Formato de red inválido");
//     let community = "public"; // Cambia si tu red usa otra comunidad SNMP
//     let timeout = Duration::from_secs(5);

//     println!("Escaneando la red para dispositivos y puertos abiertos...");

//     // Paso 1: Ejecutar RustScan
//     let rustscan_output = Command::new("rustscan")
//         .args(["-a", &network.to_string(), "--ulimit", "5000", "-r", "160-162"])
//         .output();

//     let output = match rustscan_output {
//         Ok(output) => {
//             if !output.stdout.is_empty() {
//                 println!("Salida estándar de RustScan:\n{}", String::from_utf8_lossy(&output.stdout));
//             }
//             if !output.stderr.is_empty() {
//                 eprintln!("Errores de RustScan:\n{}", String::from_utf8_lossy(&output.stderr));
//             }
//             String::from_utf8_lossy(&output.stdout).to_string()
//         }
//         Err(e) => {
//             eprintln!("Error al ejecutar RustScan: {}", e);
//             return;
//         }
//     };

//     // Paso 2: Analizar salida de RustScan
//     let devices = parse_rustscan_output(&output);
//     if devices.is_empty() {
//         println!("No se detectaron dispositivos con puertos abiertos.");
//         return;
//     }

//     println!("Dispositivos detectados:\n{:?}", devices);

//     // Paso 3: Consultar SNMP en cada dispositivo detectado
//     for (ip, ports) in devices {
//         println!("\nAnalizando dispositivo en {}...", ip);

//         for port in ports {
//             println!("  Probando puerto {}...", port);

//             match SyncSession::new(&format!("{}:{}", ip, port), community.as_bytes(), Some(timeout), 0) {
//                 Ok(mut session) => {
//                     println!("    Sesión SNMP establecida en {}:{}", ip, port);

//                     // Identificar impresoras
//                     if let Ok(description) = get_oid_string(&mut session, &[1, 3, 6, 1, 2, 1, 1, 1, 0]) {
//                         if description.to_lowercase().contains("printer") {
//                             println!("    ¡Impresora detectada! Descripción: {}", description);

//                             // Consultar más detalles específicos del Printer-MIB
//                             if let Ok(device_name) = get_oid_string(&mut session, &[1, 3, 6, 1, 2, 1, 43, 5, 1, 1, 16, 1]) {
//                                 println!("    Nombre del dispositivo: {}", device_name);
//                             }
//                         } else {
//                             println!("    No parece ser una impresora. Descripción: {}", description);
//                         }
//                     } else {
//                         println!("    No se pudo obtener información del dispositivo.");
//                     }
//                 }
//                 Err(e) => {
//                     println!("    Error al establecer sesión SNMP en el puerto {}: {}", port, e);
//                 }
//             }
//         }
//     }
// }

// /// Función para analizar la salida de RustScan y extraer IPs y puertos abiertos
// fn parse_rustscan_output(output: &str) -> Vec<(String, Vec<u16>)> {
//     let mut devices = Vec::new();
//     let mut current_ip = String::new();
//     let mut current_ports = Vec::new();

//     for line in output.lines() {
//         if line.contains("Open") {
//             if !current_ip.is_empty() {
//                 devices.push((current_ip.clone(), current_ports.clone()));
//                 current_ports.clear();
//             }
//             current_ip = line.split_whitespace().next().unwrap_or("").to_string();
//         } else if let Ok(port) = line.trim().parse::<u16>() {
//             current_ports.push(port);
//         }
//     }

//     if !current_ip.is_empty() {
//         devices.push((current_ip, current_ports));
//     }

//     devices
// }

// /// Función para obtener un OID como string
// fn get_oid_string(session: &mut SyncSession, oid: &[u32]) -> Result<String, Box<dyn Error>> {
//     let response = session.get(oid);
//     if let Some((_, Value::OctetString(value))) = response.unwrap().varbinds.into_iter().next() {
//         Ok(String::from_utf8_lossy(&value).to_string())
//     } else {
//         Err("El OID no devolvió una cadena válida.".into())
//     }
// }


use snmp::{SyncSession, Value};
use std::time::Duration;

fn main() {
    let ip = "168.176.022.140";
    let port = 161; // Puerto SNMP estándar
    let community = "public"; // Cambia si configuraste otra comunidad SNMP
    let timeout = Duration::from_secs(5);

    let address = format!("{}:{}", ip, port);
    match SyncSession::new(&address, community.as_bytes(), Some(timeout), 0) {
        Ok(mut session) => {
            println!("Sesión SNMP establecida con {}", address);

            // Consultar un OID, por ejemplo, la descripción del sistema
            let oid = &[1, 3, 6, 1, 2, 1, 1, 1, 0]; // sysDescr
            match session.get(oid) {
                Ok(response) => {
                    if let Some((_, Value::OctetString(value))) = response.varbinds.into_iter().next() {
                        println!("Descripción del sistema: {}", String::from_utf8_lossy(&value));
                    } else {
                        println!("El OID no devolvió una cadena válida.");
                    }
                }
                Err(e) => {
                    eprintln!("Error al obtener el OID: {}", e);
                }
            }
        }
        Err(e) => {
            eprintln!("Error al establecer sesión SNMP: {}", e);
        }
    }
}
