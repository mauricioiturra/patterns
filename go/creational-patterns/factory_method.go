package main

import "fmt"

// Interfaz de notificación
type Notification interface {
	Send(message string)
}

// Implementación concreta de notificación por correo electrónico
type EmailNotification struct{}

func (n *EmailNotification) Send(message string) {
	fmt.Println("Enviando correo electrónico:", message)
}

// Implementación concreta de notificación por SMS
type SMSNotification struct{}

func (n *SMSNotification) Send(message string) {
	fmt.Println("Enviando SMS:", message)
}

// Implementación concreta de notificación push
type PushNotification struct{}

func (n *PushNotification) Send(message string) {
	fmt.Println("Enviando notificación push:", message)
}

// Enumeración de tipos de notificación
type NotificationType int

const (
	EMAIL NotificationType = iota
	SMS
	PUSH
)

// Función de fábrica de notificaciones
func createNotification(t NotificationType) Notification {
	switch t {
	case EMAIL:
		return &EmailNotification{}
	case SMS:
		return &SMSNotification{}
	case PUSH:
		return &PushNotification{}
	default:
		panic("Tipo de notificación no válido")
	}
}

func main() {
	// Crear una notificación por correo electrónico
	emailNotification := createNotification(EMAIL)
	emailNotification.Send("¡Hola por correo electrónico!")

	// Crear una notificación por SMS
	smsNotification := createNotification(SMS)
	smsNotification.Send("¡Hola por SMS!")

	// Crear una notificación push
	pushNotification := createNotification(PUSH)
	pushNotification.Send("¡Hola por notificación push!")
}
