<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Delete prelist data</name>
   <tag></tag>
   <elementGuidId>1ed9e2c3-ee51-4401-982a-ad2ca5a65f7b</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <smartLocatorEnabled>false</smartLocatorEnabled>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY0NTRjNDI1M2RkMTAxN2YyODY4OTU3MiIsImNvbXBhbnkiOiI2NWVhNzhhYmY0NTgyNTc4NWI2OGEyZDciLCJjb21wYW55Q29kZSI6IjAwMHYyIiwic2Vzc2lvbiI6IjY2N2U2YzU3ZGVhZDI0ZGMxMWU1ZWZlNiIsImlhdCI6MTcxOTU2MTMwMywiZXhwIjoxNzE5NjA0NTAzfQ.2FVgKTVvAshWwiB_bpetxGz3hHyV5kd1Wrn2Y_RSsBI</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>true</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>false</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Accept</name>
      <type>Main</type>
      <value>application/json</value>
      <webElementGuid>393cc96d-4d1c-45af-b8f2-fa60e37a79c1</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpZCI6IjY0NTRjNDI1M2RkMTAxN2YyODY4OTU3MiIsImNvbXBhbnkiOiI2NWVhNzhhYmY0NTgyNTc4NWI2OGEyZDciLCJjb21wYW55Q29kZSI6IjAwMHYyIiwic2Vzc2lvbiI6IjY2N2U2YzU3ZGVhZDI0ZGMxMWU1ZWZlNiIsImlhdCI6MTcxOTU2MTMwMywiZXhwIjoxNzE5NjA0NTAzfQ.2FVgKTVvAshWwiB_bpetxGz3hHyV5kd1Wrn2Y_RSsBI</value>
      <webElementGuid>d24e1dec-ccfe-4cc6-a2ae-a0279ab1ac58</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>9.5.0</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <path></path>
   <restRequestMethod>DELETE</restRequestMethod>
   <restUrl>${disUrl}${disPr}?id=[&quot;667e6f18b0e1a89ba7c29a0b&quot;]</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.DISPOSAL_BASE_URL</defaultValue>
      <description></description>
      <id>dd023946-ab5e-4547-9987-7b5f24c05527</id>
      <masked>false</masked>
      <name>disUrl</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.disposalPrelist</defaultValue>
      <description></description>
      <id>0551c0a0-3538-4cce-a936-ada5730704f5</id>
      <masked>false</masked>
      <name>disPr</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.Options</defaultValue>
      <description></description>
      <id>a59c2ba7-668d-4d1c-a24e-ba867f1fe291</id>
      <masked>false</masked>
      <name>bt</name>
   </variables>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
