# Doctor appointment management REST API

This project was written to be submitted as the final assessment project for **Modular Monolith Course**. 
Further requirements were added after the fact to serve as an assignment project for the **Domain Driven Design Course**

## 1 Requirements & assessment criteria:
You are tasked with creating a backend system for a doctor appointment booking application. The system will be designed for a specific single doctor and should handle the logic behind managing and booking appointments. The project focuses on implementing the necessary APIs and functionality to meet the business requirements.
### 1.1 Business Requirements:
The application should have the following modules
#### 1.1.1 Doctor Availability Module:
- As a doctor, I want to be able to list my slots
- As a doctor, I want to be able to add new slots where a single time slot should have the following:
  - Id
  - Time
  - DoctorId
  - IsReserved
  - Cost
#### 1.1.2 Appointment Booking Module:
- As a Patient, I want to be able to view all doctors' available (only) slots
- As a patient, I want to be able to book an appointment in a free slot. An Appointment should have the following:
  - Id
  - SlotId
  - PatientId
  - PatientName
  - ReservedAt

#### 1.1.3  Appointment Confirmation Module:
- Once a patient schedules an appointment, the system should send a confirmation notification to the patient and the doctor
- The confirmation notification should include the appointment details, such as the patient's name, appointment time, and Doctor's name.
- For the sake of this assessment, the notification could be just a Log message
#### 1.1.4 Doctor Appointment Management:
- As a Doctor, I want to be able to view my upcoming appointments.
- As a Doctor, I want to be able to mark appointments as completed or cancel them if necessary. 

### 1.2 Data Persistence:
Use any db engine or even in-memory list with no db at all.
> I chose to go with **Mongo db**

### 1.3 Modular monolith course assessment criteria:
- You don’t need to care about authentication or authorization, make it public APIs
- Assume the system is serving a single Doctor only
- Apply modular monolith architecture
- The system should consist of four modules each with a different architecture as follows:
  - **Doctor Availability Module**: Traditional Layered Architecture
  - **Appointment Booking Module**: Clean architecture
  - Appointment Confirmation Module: Simplest architecture possible
  - **Doctor Appointment Management**: Hexagonal Architecture
- **(A plus point)** Write unit and integration testing
  